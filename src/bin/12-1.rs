use std::path::Display;

use anyhow;
use aoc23;
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
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SpringStatuses(Vec<SpringStatus>);

impl SpringStatuses {
    fn matches(&self, other: &Self) -> bool {
        use SpringStatus::*;
        // println!("{} vs {}", self, other);
        self.0.len() == other.0.len()
            && self
                .0
                .iter()
                .zip(other.0.iter())
                .all(|(s, o)| match (s, o) {
                    (Operational, Operational) => true,
                    (Damaged, Damaged) => true,
                    (Unknown, _) => true,
                    (_, Unknown) => true,
                    _ => false,
                })
    }
}

impl std::fmt::Display for SpringStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SpringStatus::*;
        for s in &self.0 {
            match *s {
                Operational => f.write_str(".")?,
                Damaged => f.write_str("#")?,
                Unknown => f.write_str("?")?,
            }
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

    fn max_operational_groups(&self) -> usize {
        self.damaged_groups() + 1
    }

    fn total_damaged(&self) -> usize {
        self.0.iter().sum()
    }

    fn total_operational(&self, len: usize) -> usize {
        //dbg!(len);
        //dbg!(self.total_damaged());
        let total_operational = len - self.total_damaged();
        //dbg!(total_operational);
        total_operational
    }

    fn possible_arrangements(&self, len: usize) -> impl Iterator<Item = SpringStatuses> {
        //println!("###");

        let mut arrangements: Vec<SpringStatuses> = Vec::new();

        let iter = std::iter::repeat(0..=self.total_operational(len))
            .take(self.max_operational_groups())
            .multi_cartesian_product();

        for operational_groups in iter {
            let operational_groups_iter = operational_groups
                .into_iter()
                .map(|group_len| (group_len, SpringStatus::Operational));
            let damaged_groups_iter = self
                .0
                .iter()
                .copied()
                .map(|group_len| (group_len, SpringStatus::Damaged));
            let mut arrangement: Vec<SpringStatus> = Vec::new();
            for (group_len, kind) in operational_groups_iter.interleave(damaged_groups_iter) {
                arrangement.extend(std::iter::repeat(kind).take(group_len));
            }
            if arrangement.len() == len {
                arrangements.push(SpringStatuses(arrangement));
            }
        }

        arrangements.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ConditionRecord {
    springs: SpringStatuses,
    damaged_groups: DamagedGroups,
}

impl ConditionRecord {
    fn is_valid_arrangement(&self, arrangement: &SpringStatuses) -> bool {
        let springs_match = self.springs.matches(arrangement);
        let arragement_damaged_groups = arrangement
            .0
            .iter()
            .group_by(|&s| *s == SpringStatus::Damaged)
            .into_iter()
            .filter_map(|(is_damaged, group)| is_damaged.then_some(group.count()))
            .collect_vec();

        let groups_match = self.damaged_groups.0 == arragement_damaged_groups;
        springs_match && groups_match
    }
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

    fn arrangements(&self) -> usize {
        self.damaged_groups
            .possible_arrangements(self.springs.0.len())
            .into_iter()
            .filter(|a| self.is_valid_arrangement(a))
            .inspect(|a| {
                // println!(
                //     "matched {} vs {} for {:?}",
                //     self.springs, a, self.damaged_groups
                // );
            })
            .count()
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let records = lines.map(|s| ConditionRecord::from_line(&s)).collect_vec();

    // TODO: use .par_iter()

    // this soltion is very imperformant, but it generates the correct answer

    let sum: usize = records.par_iter().map(|r| r.arrangements()).sum();
    println!("{}", sum);
    Ok(())
}
