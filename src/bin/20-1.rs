use std::collections::{HashMap, VecDeque};

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PulseKind {
    High,
    Low,
}

impl std::fmt::Display for PulseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PulseKind::High => "high",
            PulseKind::Low => "low",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    kind: PulseKind,
}

impl<'a> std::fmt::Display for Pulse<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.kind, self.to)
    }
}

impl<'a> Pulse<'a> {
    fn new(from: &'a str, to: &'a str, kind: PulseKind) -> Self {
        Pulse { from, to, kind }
    }
    fn low(from: &'a str, to: &'a str) -> Self {
        Pulse {
            from,
            to,
            kind: PulseKind::Low,
        }
    }
    fn high(from: &'a str, to: &'a str) -> Self {
        Pulse {
            from,
            to,
            kind: PulseKind::High,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct PulseQueue<'a> {
    inner: VecDeque<Pulse<'a>>,
    low_count: u64,
    high_count: u64,
}

impl<'a> PulseQueue<'a> {
    fn score(&self) -> u64 {
        self.low_count * self.high_count
    }
    fn send(&mut self, pulse: Pulse<'a>) {
        match pulse.kind {
            PulseKind::High => self.high_count += 1,
            PulseKind::Low => self.low_count += 1,
        };
        self.inner.push_back(pulse);
    }
    fn extend(&mut self, iter: impl IntoIterator<Item = Pulse<'a>>) {
        for pulse in iter {
            self.send(pulse);
        }
    }

    fn pop(&mut self) -> Option<Pulse<'a>> {
        self.inner.pop_front()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, PulseKind>),
    Broadcast,
    Noop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    name: &'a str,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
    kind: ModuleKind<'a>,
}

impl<'a> Module<'a> {
    fn on_demand_noop(name: &'a str, input: &'a str) -> Self {
        Module {
            name,
            inputs: vec![input],
            outputs: Vec::new(),
            kind: ModuleKind::Noop,
        }
    }

    fn process(&mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> {
        assert!(self.inputs.contains(&pulse.from));
        assert_eq!(self.name, pulse.to);
        match self.process_inner(pulse) {
            Some(kind) => self
                .outputs
                .iter()
                .map(|&to| Pulse::new(self.name, to, kind))
                .collect_vec(),
            None => Vec::new(),
        }
    }

    fn process_inner(&mut self, pulse: Pulse<'a>) -> Option<PulseKind> {
        use PulseKind::*;
        match self.kind {
            ModuleKind::FlipFlop(ref mut state) => match pulse.kind {
                // nothing happens
                High => None,
                Low => {
                    // flip flow was on
                    if *state {
                        // turn off
                        *state = !*state;
                        Some(Low)
                    // flip flow was off
                    } else {
                        // turn off
                        *state = !*state;
                        Some(High)
                    }
                }
            },
            ModuleKind::Conjunction(ref mut state) => {
                state.insert(pulse.from, pulse.kind);
                match state.values().all_equal_value() {
                    Ok(High) => Some(Low),
                    _ => Some(High),
                }
            }
            // send copy to all outputs
            ModuleKind::Broadcast => Some(pulse.kind),
            // nothing happens
            ModuleKind::Noop => None,
        }
    }
}

fn parse_module(s: &str) -> Module<'_> {
    let (kind_name, outputs) = s.split_once("->").unwrap();
    let kind_name = kind_name.trim();
    let name = kind_name.trim_start_matches(['&', '%']);

    let kind = if kind_name == "broadcaster" {
        ModuleKind::Broadcast
    } else if kind_name.starts_with('%') {
        ModuleKind::FlipFlop(false)
    } else if kind_name.starts_with('&') {
        // will be filled later
        ModuleKind::Conjunction(HashMap::new())
    } else {
        panic!("invalid module named `{}`", kind_name);
    };

    let outputs = outputs.split(',').map(|s| s.trim()).collect_vec();

    Module {
        name,
        // will be filled later
        inputs: Vec::new(),
        outputs,
        kind,
    }
}

fn parse<'a>(lines: impl IntoIterator<Item = &'a str>) -> HashMap<&'a str, Module<'a>> {
    let mut modules: HashMap<&'a str, Module<'a>> = lines
        .into_iter()
        .map(|s| parse_module(s))
        .map(|m| (m.name, m))
        .collect();

    // patch inputs from outouts
    let mut inputs: HashMap<&str, Vec<&str>> = modules
        .keys()
        .map(|&name| {
            let ins = modules
                .values()
                .filter_map(|m| m.outputs.contains(&name).then_some(m.name))
                .collect_vec();
            (name, ins)
        })
        .collect();
    for m in modules.values_mut() {
        m.inputs = inputs.remove(m.name).unwrap();
    }

    // patch inputs of conjuctions
    for m in modules.values_mut() {
        let new_state: HashMap<_, _> = m.inputs.iter().map(|&i| (i, PulseKind::Low)).collect();
        match m.kind {
            ModuleKind::Conjunction(ref mut state) => *state = new_state,
            _ => {}
        }
    }

    // allow broadcaster to recieve pulses from button
    modules
        .get_mut("broadcaster")
        .unwrap()
        .inputs
        .push("button");

    modules
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines().collect_vec();
    let mut modules = parse(lines.iter().map(|s| s.as_str()));

    // println!("Modules: {:#?}", &modules);

    let mut queue = PulseQueue::default();

    for i in 0..1_000 {
        // println!("### iteration {} pulses:", i);
        queue.send(Pulse::low("button", "broadcaster"));
        while let Some(pulse) = queue.pop() {
            // println!("{}", &pulse);
            // found output without inputs to be noops
            let mut noop = Module::on_demand_noop(pulse.to, pulse.from);
            let module = modules.get_mut(pulse.to).unwrap_or(&mut noop);
            let generated = module.process(pulse);
            queue.extend(generated);
        }
    }

    let sum: u64 = queue.score();
    println!("{}", sum);
    Ok(())
}
