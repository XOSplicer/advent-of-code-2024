use std::collections::{HashMap, HashSet};

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn get(&self, cat: Category) -> u16 {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct LoopError;

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

    fn apply(&self, part: &Part) -> Option<RuleOutcome<'a>> {
        let part_value = part.get(self.category);
        match self.comp {
            Comp::Greater => (part_value > self.value).then_some(self.outcome),
            Comp::Less => (part_value < self.value).then_some(self.outcome),
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

    fn apply(&self, part: &Part) -> RuleOutcome<'a> {
        self.rules
            .iter()
            .find_map(|rule| rule.apply(part))
            .unwrap_or(self.final_rule)
    }
}

#[derive(Debug, Clone)]
struct Workflows<'a>(HashMap<&'a str, Workflow<'a>>);

impl<'a> Workflows<'a> {
    fn apply(&self, part: &Part) -> Result<RuleOutcome, LoopError> {
        let mut seen = HashSet::with_capacity(self.0.len());
        let mut w_name = "in";
        seen.insert(w_name);
        loop {
            let mut workflow = self.0.get(w_name).unwrap();
            match workflow.apply(part) {
                RuleOutcome::Accepted => {
                    return Ok(RuleOutcome::Accepted);
                }
                RuleOutcome::Rejected => {
                    return Ok(RuleOutcome::Rejected);
                }
                RuleOutcome::NextWorkflow(next) => {
                    if seen.contains(next) {
                        return Err(LoopError);
                    } else {
                        w_name = next;
                        seen.insert(w_name);
                    }
                }
            }
        }
    }
}

impl Part {
    fn from_line(line: &str) -> Self {
        let line = line.trim().trim_start_matches('{').trim_end_matches('}');
        let mut parts = line
            .split(',')
            .map(|s| s.split_at(2).1)
            .map(|s| s.parse::<u16>().unwrap());
        Part {
            x: parts.next().unwrap(),
            m: parts.next().unwrap(),
            a: parts.next().unwrap(),
            s: parts.next().unwrap(),
        }
    }

    fn rating(&self) -> u32 {
        self.x as u32 + self.m as u32 + self.a as u32 + self.s as u32
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

    let parts = lines.map(|s| Part::from_line(s)).collect_vec();

    let sum: u32 = parts.into_iter().filter_map(|part| match workflows.apply(&part) {
        Ok(RuleOutcome::Accepted) => Some(part.rating()),
        _ => None,
    }).sum();

    println!("{}", sum);
    Ok(())
}
