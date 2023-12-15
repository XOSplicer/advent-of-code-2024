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

    fn flip(&mut self) {
        match self {
            &mut Entry::Ash => *self = Entry::Rock,
            &mut Entry::Rock => *self = Entry::Ash,
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

    fn vertical_line_of_reflection(&self, ignore_row: Option<usize>) -> Option<usize> {
        major_axis_symmetry(&self.0, ignore_row)
    }

    fn smudge_at(&mut self, row: usize, col: usize) {
        self.0[row][col].flip();
    }
}

fn major_axis_symmetry(pattern: &Vec<Vec<Entry>>, ignore: Option<usize>) -> Option<usize> {
    for i in 0..pattern[0].len() - 1 {
        if Some(i) == ignore {
            continue;
        }
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

impl RowMajorPattern {
    fn dbg_print(&self) {
        for v in self.0.iter() {
            for c in v {
                print!("{}", c);
            }
            println!("");
        }
    }
}

impl ColMajorPattern {
    fn horizontal_line_of_reflection(&self, ignore_col: Option<usize>) -> Option<usize> {
        major_axis_symmetry(&self.0, ignore_col)
    }
}

fn pattern_value(
    pattern: &RowMajorPattern,
    ignore_row: Option<usize>,
    ignore_col: Option<usize>,
) -> usize {
    let row_major = pattern;
    let col_major = pattern.to_col_major();
    let value = row_major
        .vertical_line_of_reflection(ignore_row)
        .unwrap_or(0)
        + col_major
            .horizontal_line_of_reflection(ignore_col)
            .unwrap_or(0)
            * 100;
    value
}

fn smudge_patter_iter(pattern: &RowMajorPattern) -> impl Iterator<Item = RowMajorPattern> + '_ {
    (0..pattern.0.len())
        .cartesian_product(0..pattern.0[0].len())
        .map(|(row, col)| {
            let mut p = pattern.clone();
            p.smudge_at(row, col);
            p
        })
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
            let original_value = pattern_value(&pattern, None, None);
            pattern.dbg_print();
            dbg!(original_value);

            let orig_row = original_value % 100;
            let orig_col = original_value / 100;

            let ignore_row = (orig_row != 0).then(|| orig_row - 1);
            let ignore_col = (orig_col != 0).then(|| orig_col - 1);

            let (new_value, new_pattern) = smudge_patter_iter(&pattern)
                .map(|smudged| (pattern_value(&smudged, ignore_row, ignore_col), smudged))
                .inspect(|(v, _p)| {
                    dbg!(v);
                })
                .filter(|&(v, _)| v > 0 && v != original_value)
                .next()
                .unwrap();
            new_pattern.dbg_print();
            dbg!(new_value);

            let new_row = new_value % 100;
            let new_col = new_value / 100;

            let fixed_row = if orig_row == new_row { 0 } else { new_row };
            let fixed_col = if orig_col == new_col { 0 } else { new_col };

            let fixed_value = 100 * fixed_col + fixed_row;
            dbg!(fixed_value);
            println!("---------------");
            fixed_value
        })
        .sum();

    println!("{}", sum);
    Ok(())
}
