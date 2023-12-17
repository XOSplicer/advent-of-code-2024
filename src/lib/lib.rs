#[allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input_lines() -> impl Iterator<Item = String> {
    let input_file = std::env::args()
        .skip(1)
        .next()
        .expect("Expected input FILE");
    let lines = BufReader::new(File::open(input_file).expect("Could not open FILE"))
        .lines()
        .map(|line| line.expect("Could not read line"));
    lines
}

pub fn read_file_lines(file: &str) -> impl Iterator<Item = String> {
    let input_file = file;
    let lines = BufReader::new(File::open(input_file).expect("Could not open FILE"))
        .lines()
        .map(|line| line.expect("Could not read line"));
    lines
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub row: isize,
    pub col: isize,
}

impl Location {
    #[inline(always)]
    pub fn new(row: isize, col: isize) -> Self {
        Location { row, col }
    }

    #[inline(always)]
    pub fn new_usize(row: usize, col: usize) -> Self {
        Location {
            row: row as isize,
            col: col as isize,
        }
    }

    #[inline(always)]
    pub fn up(&self) -> Location {
        Location {
            row: self.row - 1,
            col: self.col,
        }
    }

    #[inline(always)]
    pub fn down(&self) -> Location {
        Location {
            row: self.row + 1,
            col: self.col,
        }
    }

    #[inline(always)]
    pub fn right(&self) -> Location {
        Location {
            row: self.row,
            col: self.col + 1,
        }
    }

    #[inline(always)]
    pub fn left(&self) -> Location {
        Location {
            row: self.row,
            col: self.col - 1,
        }
    }

    #[inline(always)]
    pub fn north(&self) -> Location {
        self.up()
    }

    #[inline(always)]
    pub fn south(&self) -> Location {
        self.down()
    }

    #[inline(always)]
    pub fn east(&self) -> Location {
        self.right()
    }

    #[inline(always)]
    pub fn west(&self) -> Location {
        self.left()
    }

    #[inline(always)]
    pub fn apply(&self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            Up => self.up(),
            Down => self.down(),
            Right => self.right(),
            Left => self.left(),
        }
    }

    #[inline(always)]
    pub fn apply_n(&self, dir: Direction, n: usize) -> Self {
        let mut s = *self;
        for _ in 0..n {
            s = s.apply(dir);
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    #[inline(always)]
    pub fn up() -> Direction {
        Direction::Up
    }

    #[inline(always)]
    pub fn down() -> Direction {
        Direction::Down
    }

    #[inline(always)]
    pub fn right() -> Direction {
        Direction::Right
    }

    #[inline(always)]
    pub fn left(&self) -> Direction {
        Direction::Left
    }

    #[inline(always)]
    pub fn north() -> Direction {
        Direction::Up
    }

    #[inline(always)]
    pub fn south() -> Direction {
        Direction::Down
    }

    #[inline(always)]
    pub fn east() -> Direction {
        Direction::Right
    }

    #[inline(always)]
    pub fn west(&self) -> Direction {
        Direction::Left
    }

    #[inline(always)]
    pub fn rev(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}
