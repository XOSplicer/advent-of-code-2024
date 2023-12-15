use anyhow;
use aoc23;
use itertools::*;

fn hash(s: &str) -> usize {
    assert!(s.is_ascii());
    s.trim()
        .bytes()
        .fold(0_usize, |acc, x| ((acc + x as usize) * 17) % 256)
}

#[derive(Debug, Clone)]
struct HashMap<'a> {
    boxes: Box<[Vec<HashMapEntry<'a>>; 256]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HashMapEntry<'a> {
    label: &'a str,
    value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Insert(u8),
    Remove,
}

struct Step<'a> {
    label: &'a str,
    operation: Operation,
}

impl<'a> Step<'a> {
    fn from_str(s: &'a str) -> Self {
        let sep = if s.contains('=') {
            '='
        } else if s.contains('-') {
            '-'
        } else {
            panic!("invalid step: {}", s);
        };
        let mut parts = s.split(sep);
        let label = parts.next().unwrap();
        let operation = if sep == '=' {
            let value: u8 = parts.next().unwrap().parse().unwrap();
            Operation::Insert(value)
        } else {
            Operation::Remove
        };

        Step { label, operation }
    }
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        HashMap {
            boxes: Box::new(std::array::from_fn(|_| Vec::new())),
        }
    }

    fn insert(&mut self, label: &'a str, value: u8) {
        let h = hash(label);
        let v = &mut self.boxes[h];
        // entry already present
        if let Some(entry) = v.iter_mut().find(|entry| entry.label == label) {
            entry.value = value;
        // entry not present
        } else {
            v.push(HashMapEntry { label, value })
        }
    }

    fn remove(&mut self, label: &str) {
        let h = hash(label);
        let v = &mut self.boxes[h];
        if let Some((idx, _)) = v.iter().find_position(|entry| entry.label == label) {
            v.remove(idx);
        }
    }

    fn focus_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_idx, v)| {
                v.iter()
                    .enumerate()
                    .map(|(lense_idx, entry)| {
                        (box_idx + 1) * (lense_idx + 1) * entry.value as usize
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines();
    let line = lines.next().unwrap();
    let steps = line.split(',').map(|s| Step::from_str(s));
    let mut hashmap = HashMap::new();
    for step in steps {
        match step.operation {
            Operation::Insert(value) => {
                hashmap.insert(step.label, value);
            }
            Operation::Remove => hashmap.remove(step.label),
        }
    }

    println!("{}", hashmap.focus_power());
    Ok(())
}
