use core::fmt;
use std::{fmt::write, path::Display};

use anyhow;
use aoc23;
use cached::proc_macro::cached;
use itertools::*;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

impl SpringStatus {
    fn from_char(c: char) -> Self {
        match c {
            '.' => SpringStatus::Operational,
            '#' => SpringStatus::Damaged,
            '?' => SpringStatus::Unknown,
            _ => panic!("invalid spring status: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match *self {
            SpringStatus::Operational => '.',
            SpringStatus::Damaged => '#',
            SpringStatus::Unknown => '?',
        }
    }
}

impl std::fmt::Display for SpringStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SpringStatuses(Vec<SpringStatus>);

impl std::fmt::Display for SpringStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SpringStatus::*;
        for s in &self.0 {
            s.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct DamagedGroups(Vec<usize>);

impl DamagedGroups {
    fn damaged_groups(&self) -> usize {
        self.0.len()
    }

    fn total_damaged(&self) -> usize {
        self.0.iter().sum()
    }

    fn total_operational(&self, len: usize) -> usize {
        len - self.total_damaged()
    }
}

impl fmt::Display for DamagedGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().join(","))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ConditionRecord {
    springs: SpringStatuses,
    damaged_groups: DamagedGroups,
}

impl ConditionRecord {
    fn from_line(line: &str) -> Self {
        let mut parts = line.trim().split_whitespace();
        let springs = parts
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| SpringStatus::from_char(c))
            .collect_vec();
        let damaged_groups = parts
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_vec();
        ConditionRecord {
            springs: SpringStatuses(springs),
            damaged_groups: DamagedGroups(damaged_groups),
        }
    }

    fn arrangements(self) -> u64 {
        arrangements(self)
    }
}

impl SpringStatuses {
    fn trim_operational(self) -> Self {
        let mut front_trimmed = self
            .0
            .into_iter()
            .skip_while(|&s| s == SpringStatus::Operational)
            .collect_vec();
        front_trimmed.reverse();
        let mut trimmed = front_trimmed
            .into_iter()
            .skip_while(|&s| s == SpringStatus::Operational)
            .collect_vec();
        trimmed.reverse();

        SpringStatuses(trimmed)
    }

    fn contains_damaged(&self) -> bool {
        self.0.contains(&SpringStatus::Damaged)
    }
}

impl ConditionRecord {
    fn trim_operational(mut self) -> Self {
        self.springs = self.springs.trim_operational();
        self
    }

    fn is_final(&self) -> bool {
        self.damaged_groups.0.is_empty() && !self.springs.contains_damaged()
    }

    fn is_impossible(&self) -> bool {
        (self.damaged_groups.0.is_empty() && self.springs.contains_damaged())
            || (self.springs.0.len()
                < self.damaged_groups.total_damaged() + self.damaged_groups.damaged_groups() - 1)
    }
}

impl DamagedGroups {
    fn pop_first(self) -> Self {
        DamagedGroups(self.0.into_iter().skip(1).collect_vec())
    }
}

impl SpringStatuses {
    fn pop_first_group(self, group: usize) -> Self {
        SpringStatuses(self.0.into_iter().skip(group).collect_vec())
    }
}

#[cached]
fn arrangements(record: ConditionRecord) -> u64 {
    let record = record.trim_operational();

    if record.is_final() {
        return 1;
    }

    if record.is_impossible() {
        return 0;
    }

    let first = record.springs.0[0];
    if first == SpringStatus::Damaged {
        // match first group
        let first_group_len = record
            .springs
            .0
            .iter()
            .take_while(|&s| *s == SpringStatus::Damaged || *s == SpringStatus::Unknown)
            .count();
        if first_group_len >= record.damaged_groups.0[0] {
            // pop first group
            let mut popped_rec = ConditionRecord {
                damaged_groups: record.damaged_groups.clone().pop_first(),
                springs: record
                    .springs
                    .clone()
                    .pop_first_group(record.damaged_groups.0[0]),
            };

            // if it starts with damaged the group was incorrectly popped
            if popped_rec.springs.0.first() == Some(&SpringStatus::Damaged) {
                return 0;
            }

            // force first to be operational, as it may not be damaged, otherwise the group would not have matched
            if let Some(f) = popped_rec.springs.0.first_mut() {
                *f = SpringStatus::Operational
            }

            return arrangements(popped_rec);
        } else {
            return 0;
        }
    } else if first == SpringStatus::Unknown {
        let mut rec1 = record.clone();
        rec1.springs.0[0] = SpringStatus::Damaged;
        let mut rec2 = record.clone();
        rec2.springs.0[0] = SpringStatus::Operational;
        return arrangements(rec1) + arrangements(rec2);
    } else {
        panic!("Unreachable first spring status of Operational")
    }
}

impl std::fmt::Display for ConditionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", &self.springs, &self.damaged_groups)
    }
}

impl ConditionRecord {
    fn unfold(self) -> Self {
        let n = 5;
        let springs = std::iter::repeat_with(|| self.springs.0.clone())
            .take(n)
            .interleave(std::iter::repeat_with(|| vec![SpringStatus::Unknown]).take(n - 1))
            .flatten()
            .collect_vec();
        let damaged_groups = std::iter::repeat_with(|| self.damaged_groups.0.clone())
            .take(n)
            .flatten()
            .collect_vec();
        ConditionRecord {
            springs: SpringStatuses(springs),
            damaged_groups: DamagedGroups(damaged_groups),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let records = lines
        //.map(|s| ConditionRecord::from_line(&s).unfold())
        .map(|s| ConditionRecord::from_line(&s).unfold())
        .inspect(|s| println!("{:?}", &s))
        .collect_vec();

    let sum: u64 = records.into_iter().map(|r| r.arrangements()).sum();
    println!("{}", sum);
    Ok(())
}
