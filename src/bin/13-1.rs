use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Entry {
    Ash,
    Rock,
}

impl Entry {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Entry::Ash,
            '#' => Entry::Rock,
            _ => panic!("invalid entry: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match *self {
            Entry::Ash => '.',
            Entry::Rock => '#',
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

    fn vertical_line_of_reflection(&self) -> Option<usize> {
        major_axis_symmetry(&self.0)
    }
}

fn major_axis_symmetry(pattern: &Vec<Vec<Entry>>) -> Option<usize> {
    for i in 0..pattern[0].len() - 1 {
        let has_symmetry = pattern
            .iter()
            .all(|axis| has_single_axis_symmetry_at(axis, i));
        if has_symmetry {
            return Some(i + 1);
        }
    }
    None
}

fn has_single_axis_symmetry_at(axis: &Vec<Entry>, pos: usize) -> bool {
    if pos >= axis.len() / 2 {
        let mut axis_rev = axis.clone();
        axis_rev.reverse();
        let pos_rev = axis.len() - pos - 2;
        return has_single_axis_symmetry_at(&axis_rev, pos_rev);
    }

    // dbg!(axis); dbg!(pos); dbg!(axis.len());

    let slice = &axis[0..=(2 * pos + 1)];

    let is_symmetric = slice.iter().zip(slice.iter().rev()).all(|(l, r)| l == r);
    is_symmetric
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ColMajorPattern(Vec<Vec<Entry>>);

impl ColMajorPattern {
    fn horizontal_line_of_reflection(&self) -> Option<usize> {
        major_axis_symmetry(&self.0)
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines().peekable();

    let mut patterns = Vec::new();
    while lines.peek().is_some() {
        patterns.push(RowMajorPattern::from_lines(&mut lines));
    }

    let sum: usize = patterns
        .iter()
        .map(|pattern| {
            let row_major = pattern;
            let col_major = pattern.to_col_major();
            /*
            for v in row_major.0.iter() {
                for c in v {
                    print!("{}", c);
                }
                println!("");
            }
            println!("---");
            for v in col_major.0.iter() {
                for c in v {
                    print!("{}", c);
                }
                println!("");
            }
            */
            let value = row_major.vertical_line_of_reflection().unwrap_or(0)
                + col_major.horizontal_line_of_reflection().unwrap_or(0) * 100;
            // println!("{}", value);
            value
        })
        .sum();

    println!("{}", sum);
    Ok(())
}
