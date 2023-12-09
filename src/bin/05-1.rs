use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone)]
struct Seeds(Vec<u64>);

impl Seeds {
    fn from_line(line: &str) -> Self {
        let inner = line
            .strip_prefix("seeds:")
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();
        Seeds(inner)
    }
}

#[derive(Debug, Clone, Copy)]
struct RangeMapPart {
    destination_start: u64,
    source_start: u64,
    range_len: u64,
}

impl RangeMapPart {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split_whitespace().map(|p| p.parse().unwrap());
        RangeMapPart {
            destination_start: parts.next().unwrap(),
            source_start: parts.next().unwrap(),
            range_len: parts.next().unwrap(),
        }
    }

    fn map_if_contained(&self, from: u64) -> Option<u64> {
        if self.source_start <= from && from < self.source_start + self.range_len {
            Some(self.destination_start + from - self.source_start)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct RangeMap {
    from_category: String,
    to_category: String,
    ranges: Vec<RangeMapPart>,
}

impl RangeMap {
    fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        let title = lines.next().unwrap();
        let mut categories = title.split_whitespace().next().unwrap().clone().split('-');
        let from_category = categories.next().unwrap().to_owned();
        categories.next(); // skip 'to'
        let to_category = categories.next().unwrap().to_owned();
        let mut ranges = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            ranges.push(RangeMapPart::from_line(&line));
        }
        RangeMap {
            from_category,
            to_category,
            ranges,
        }
    }

    fn map(&self, from: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| r.map_if_contained(from))
            .unwrap_or(from)
    }
}

#[derive(Debug, Clone)]
struct RangeMaps(Vec<RangeMap>);

impl RangeMaps {
    fn map(&self, seed: u64) -> u64 {
        self.0.iter().fold(seed, |curr, rm| rm.map(curr))
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines().peekable();
    // let mut lines = aoc23::read_input_lines().peekable();
    let seeds = Seeds::from_line(&lines.next().unwrap());
    lines.next(); // skip blank
    let mut maps = RangeMaps(Vec::new());
    while lines.peek().is_some() {
        maps.0.push(RangeMap::from_lines(&mut lines));
    }

    println!("{:?}", &seeds);
    println!("{:#?}", &maps);

    let mapped = seeds.0.iter().map(|seed| maps.map(*seed)).collect_vec();

    println!("mapped: {:#?}", &mapped);

    let min = mapped.iter().min().unwrap();

    println!("{}", min);
    Ok(())
}
