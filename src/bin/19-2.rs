use std::collections::HashMap;

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    final_rule: RuleOutcome<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Comp {
    Greater,
    Less,
}

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    category: Category,
    value: u16,
    comp: Comp,
    outcome: RuleOutcome<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleOutcome<'a> {
    Accepted,
    Rejected,
    NextWorkflow(&'a str),
}

impl<'a> RuleOutcome<'a> {
    fn from_str(s: &'a str) -> Self {
        match s {
            "A" => RuleOutcome::Accepted,
            "R" => RuleOutcome::Rejected,
            _ => RuleOutcome::NextWorkflow(s),
        }
    }
}

impl<'a> Rule<'a> {
    fn from_str(s: &'a str) -> Rule<'a> {
        let (left, outcome) = s.split_once(':').unwrap();
        let outcome = RuleOutcome::from_str(outcome);
        let (cat_comp, value) = left.split_at(2);
        let value: u16 = value.parse().unwrap();
        let category = match cat_comp.chars().nth(0).unwrap() {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            s => panic!("invalid field: {}", s),
        };
        let comp = match cat_comp.chars().nth(1).unwrap() {
            '>' => Comp::Greater,
            '<' => Comp::Less,
            s => panic!("invalid comp: {}", s),
        };

        Rule {
            category,
            comp,
            value,
            outcome,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    from: u16,
    to_excl: u16,
}

impl Range {
    fn new(from: u16, to_excl: u16) -> Self {
        assert!(from <= to_excl);
        Range { from, to_excl }
    }

    fn contains(&self, value: u16) -> bool {
        self.from <= value && value < self.to_excl
    }

    fn is_empty(&self) -> bool {
        self.from == self.to_excl
    }

    /// splits (from..to_excl) to (from..at), (at..to_excl)
    fn split(&self, at: u16) -> (Self, Self) {
        if self.contains(at) {
            (Range::new(self.from, at), Range::new(at, self.to_excl))
        } else if at < self.from {
            (Range::new(0, 0), *self)
        } else {
            (*self, Range::new(0, 0))
        }
    }

    fn len(&self) -> u16 {
        self.to_excl - self.from
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XMASRanges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl XMASRanges {
    fn get(&self, cat: Category) -> Range {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
    fn set(&self, cat: Category, range: Range) -> Self {
        match cat {
            Category::X => XMASRanges { x: range, ..*self },
            Category::M => XMASRanges { m: range, ..*self },
            Category::A => XMASRanges { a: range, ..*self },
            Category::S => XMASRanges { s: range, ..*self },
        }
    }
    fn combinations(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuleResult<'a> {
    matched_ranges: XMASRanges,
    matched_outcome: RuleOutcome<'a>,
    unmatched_ranges: XMASRanges,
}

impl<'a> Rule<'a> {
    fn apply(&self, xmas: &XMASRanges) -> RuleResult<'a> {
        let split_value = match self.comp {
            Comp::Less => self.value,
            Comp::Greater => self.value + 1,
        };
        let (new_range_lower, new_range_upper) = xmas.get(self.category).split(split_value);
        let new_xmas_lower = xmas.set(self.category, new_range_lower);
        let new_xmas_upper = xmas.set(self.category, new_range_upper);
        let (matched_ranges, unmatched_ranges) = match self.comp {
            Comp::Less => (new_xmas_lower, new_xmas_upper),
            Comp::Greater => (new_xmas_upper, new_xmas_lower),
        };
        RuleResult {
            matched_ranges,
            unmatched_ranges,
            matched_outcome: self.outcome,
        }
    }
}

impl<'a> Workflow<'a> {
    fn from_line(line: &'a str) -> Workflow<'a> {
        let (name, rest) = line.split_once('{').unwrap();
        let rest = rest.trim_end_matches('}');
        let mut parts = rest.split(',');
        let rules: Vec<Rule<'a>> = parts
            .take_while_ref(|s| s.contains(':'))
            .map(|s| Rule::from_str(s))
            .collect_vec();
        let final_rule = parts.next().map(|s| RuleOutcome::from_str(s)).unwrap();

        Workflow {
            name,
            rules,
            final_rule,
        }
    }

    fn apply(&self, ranges: XMASRanges) -> Vec<(XMASRanges, RuleOutcome<'a>)> {
        let mut matched = Vec::new();
        let mut unmatched = ranges;
        for rule in &self.rules {
            let res = rule.apply(&unmatched);
            matched.push((res.matched_ranges, res.matched_outcome));
            unmatched = res.unmatched_ranges;
        }
        matched.push((unmatched, self.final_rule));
        matched
    }
}

#[derive(Debug, Clone)]
struct Workflows<'a>(HashMap<&'a str, Workflow<'a>>);

impl<'a> Workflows<'a> {
    fn apply(&self, ranges: XMASRanges) -> Vec<(XMASRanges, RuleOutcome<'a>)> {
        let mut worklist = vec![(ranges, "in")];
        let mut finished = Vec::new();
        while let Some((r1, w_name)) = worklist.pop() {
            let v = self.0.get(w_name).unwrap().apply(r1);
            for (r2, outcome) in v {
                match outcome {
                    RuleOutcome::Accepted | RuleOutcome::Rejected => finished.push((r2, outcome)),
                    RuleOutcome::NextWorkflow(next) => worklist.push((r2, next)),
                };
            }
        }
        finished
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines().collect_vec();
    let mut lines = lines.iter();
    let mut workflows = HashMap::new();
    while let Some(line) = lines.next() {
        if line.trim() == "" {
            break;
        }
        let w = Workflow::from_line(&line);
        workflows.insert(w.name, w);
    }
    let workflows = Workflows(workflows);

    let start_xmas = XMASRanges {
        x: Range::new(1, 4001),
        m: Range::new(1, 4001),
        a: Range::new(1, 4001),
        s: Range::new(1, 4001),
    };
    let ranges = workflows.apply(start_xmas);

    let sum: u64 = ranges
        .into_iter()
        .filter_map(|(xmas, outcome)| {
            (outcome == RuleOutcome::Accepted).then_some(xmas.combinations())
        })
        .sum();
    println!("{}", sum);
    Ok(())
}
