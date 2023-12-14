use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, fmt, fmt::write, path::Display};

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Entry {
    Round,
    Cube,
    Empty,
}

impl Entry {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Entry::Empty,
            'O' => Entry::Round,
            '#' => Entry::Cube,
            _ => panic!("invalid entry: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match *self {
            Entry::Empty => '.',
            Entry::Round => 'O',
            Entry::Cube => '#',
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct RowMajorPattern(Vec<Vec<Entry>>);

impl RowMajorPattern {
    fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        let inner = lines
            .take_while(|s| !s.trim().is_empty())
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| Entry::from_char(c))
                    .collect_vec()
            })
            .collect_vec();
        RowMajorPattern(inner)
    }

    fn to_col_major(&self) -> ColMajorPattern {
        let mut cols = Vec::with_capacity(self.0[0].len());
        for c in 0..self.0[0].len() {
            let col = self.0.iter().map(|row| row[c]).collect_vec();
            cols.push(col);
        }
        ColMajorPattern(cols)
    }

    fn dbg_print(&self) {
        for v in self.0.iter() {
            for c in v {
                print!("{}", c);
            }
            println!("");
        }
    }

    fn tilt_west(&mut self) {
        for row in &mut self.0 {
            vec_tilt_to_start(row);
        }
    }

    fn tilt_east(&mut self) {
        for row in &mut self.0 {
            row.reverse();
            vec_tilt_to_start(row);
            row.reverse();
        }
    }

    fn northern_total_load(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                (self.0.len() - row_idx) * row.iter().filter(|&e| *e == Entry::Round).count()
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ColMajorPattern(Vec<Vec<Entry>>);

impl ColMajorPattern {
    fn to_row_major(&self) -> RowMajorPattern {
        let mut rows = Vec::with_capacity(self.0[0].len());
        for r in 0..self.0[0].len() {
            let row = self.0.iter().map(|col| col[r]).collect_vec();
            rows.push(row);
        }
        RowMajorPattern(rows)
    }

    fn tilt_north(&mut self) {
        for col in &mut self.0 {
            vec_tilt_to_start(col);
        }
    }

    fn tilt_south(&mut self) {
        for col in &mut self.0 {
            col.reverse();
            vec_tilt_to_start(col);
            col.reverse();
        }
    }
}

fn vec_tilt_to_start(v: &mut Vec<Entry>) {
    let orig = v.clone();
    for (orig_idx, e) in orig
        .into_iter()
        .enumerate()
        .filter(|&(_, e)| e == Entry::Round)
    {
        // find free spot
        let mut free_idx = orig_idx;
        while free_idx > 0 {
            if v[free_idx - 1] == Entry::Empty {
                free_idx -= 1;
            } else {
                break;
            }
        }

        // replace orig_idx with empty
        v[orig_idx] = Entry::Empty;
        // place round at free_idx
        v[free_idx] = Entry::Round;
    }
}

fn single_cycle(pattern: RowMajorPattern) -> RowMajorPattern {
    let mut pattern = pattern.to_col_major();
    pattern.tilt_north();
    let mut pattern = pattern.to_row_major();
    pattern.tilt_west();
    let mut pattern = pattern.to_col_major();
    pattern.tilt_south();
    let mut pattern = pattern.to_row_major();
    pattern.tilt_east();
    pattern
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HashList {
    /// hash of seen pattern to index in values
    hashes: HashMap<u64, usize>,
    /// load value for each seen pattern
    values: Vec<usize>,
}

impl HashList {
    fn width_capacity(capacity: usize) -> Self {
        HashList {
            hashes: HashMap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, pattern: &RowMajorPattern) {
        let value = pattern.northern_total_load();
        self.values.push(value);
        let hash = Self::hash(pattern);
        self.hashes.insert(hash, self.values.len() - 1);
    }

    fn hash(pattern: &RowMajorPattern) -> u64 {
        let mut s = DefaultHasher::new();
        pattern.hash(&mut s);
        s.finish()
    }

    fn get(&self, pattern: &RowMajorPattern) -> Option<HashListEntry> {
        let hash = Self::hash(pattern);
        self.hashes.get(&hash).copied().map(|idx| HashListEntry {
            idx,
            value: self.values[idx],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HashListEntry {
    idx: usize,
    value: usize,
}

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines().peekable();
    let pattern = RowMajorPattern::from_lines(&mut lines);
    pattern.dbg_print();

    let iterations = 1_000_000_000_usize;
    let mut hashlist = HashList::width_capacity(iterations / 1000);

    let mut pattern = pattern;
    for current_iteration in 0..iterations {
        pattern = single_cycle(pattern);
        if let Some(hle) = hashlist.get(&pattern) {
            println!("found reoccuring pattern: hashlist entry: {:?}", &hle);
            let ring_length = current_iteration - hle.idx;
            let final_pattern_index = hle.idx + (iterations - hle.idx) % ring_length - 1;
            dbg!(current_iteration);
            dbg!(ring_length);
            dbg!(final_pattern_index);
            let final_value = hashlist.values[final_pattern_index];
            // dbg!(hashlist.values[final_pattern_index-1]);
            // dbg!(hashlist.values[final_pattern_index]);
            // dbg!(hashlist.values[final_pattern_index+1]);

            println!("final value: {}", final_value);
            // pattern.dbg_print();
            break;
        }
        hashlist.push(&pattern);
        if (current_iteration % 1_000_000 == 0) {
            println!("progress {} / 1000", current_iteration / 1000000);
        }
    }

    /*
    let mut pattern = pattern.to_col_major();
    pattern.tilt_north();
    let pattern = pattern.to_row_major();
    */

    // let sum: usize = pattern.northern_total_load();

    //println!("----");
    //pattern.dbg_print();

    // println!("{}", sum);
    Ok(())
}
