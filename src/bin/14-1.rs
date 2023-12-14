use std::{fmt, fmt::write, path::Display};

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
            Self::col_tilt_north(col);
        }
    }

    fn col_tilt_north(col: &mut Vec<Entry>) {
        let orig = col.clone();
        for (orig_idx, e) in orig
            .into_iter()
            .enumerate()
            .filter(|&(_, e)| e == Entry::Round)
        {
            // find free spot
            let mut free_idx = orig_idx;
            while free_idx > 0 {
                if col[free_idx - 1] == Entry::Empty {
                    free_idx -= 1;
                } else {
                    break;
                }
            }

            // replace orig_idx with empty
            col[orig_idx] = Entry::Empty;
            // place round at free_idx
            col[free_idx] = Entry::Round;
        }
    }

    fn northern_total_load(&self) -> usize {
        self.0
            .iter()
            .map(|col| {
                col.iter()
                    .enumerate()
                    .filter_map(|(idx, &e)| (e == Entry::Round).then_some(col.len() - idx))
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines().peekable();
    let pattern = RowMajorPattern::from_lines(&mut lines);
    pattern.dbg_print();
    let mut pattern = pattern.to_col_major();
    pattern.tilt_north();

    let sum: usize = pattern.northern_total_load();

    pattern.to_row_major().dbg_print();

    println!("{}", sum);
    Ok(())
}
