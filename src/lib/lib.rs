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
    /// Create a new location at (row, col)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location, Location { row: 1, col: 1});
    /// ```
    #[inline(always)]
    pub fn new(row: isize, col: isize) -> Self {
        Location { row, col }
    }

    /// Create a new location at (row, col), converting from usize using `as`
    ///
    /// panics if row or col can not be converted to usize
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new_usize(1, 1);
    /// assert_eq!(location, Location { row: 1, col: 1});
    /// ```
    #[inline(always)]
    pub fn new_usize(row: usize, col: usize) -> Self {
        Location {
            row: row as isize,
            col: col as isize,
        }
    }

    /// Create a new location at (row-1, col)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.up(), Location { row: 0, col: 1});
    /// ```
    #[inline(always)]
    pub fn up(&self) -> Location {
        Location {
            row: self.row - 1,
            col: self.col,
        }
    }

    /// Create a new location at (row+1, col)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.down(), Location { row: 2, col: 1});
    /// ```
    #[inline(always)]
    pub fn down(&self) -> Location {
        Location {
            row: self.row + 1,
            col: self.col,
        }
    }

    /// Create a new location at (row, col+1)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.right(), Location { row: 1, col: 2});
    /// ```
    #[inline(always)]
    pub fn right(&self) -> Location {
        Location {
            row: self.row,
            col: self.col + 1,
        }
    }

    /// Create a new location at (row, col-1)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.left(), Location { row: 1, col: 0});
    /// ```
    #[inline(always)]
    pub fn left(&self) -> Location {
        Location {
            row: self.row,
            col: self.col - 1,
        }
    }

    /// Create a new location at (row-1, col)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.north(), Location { row: 0, col: 1});
    /// ```
    #[inline(always)]
    pub fn north(&self) -> Location {
        self.up()
    }

    /// Create a new location at (row+1, col)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.south(), Location { row: 2, col: 1});
    /// ```
    #[inline(always)]
    pub fn south(&self) -> Location {
        self.down()
    }

    /// Create a new location at (row, col+1)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.east(), Location { row: 1, col: 2});
    /// ```
    #[inline(always)]
    pub fn east(&self) -> Location {
        self.right()
    }

    /// Create a new location at (row, col-1)
    ///
    /// ```
    /// # use aoc23::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.west(), Location { row: 1, col: 0});
    /// ```
    #[inline(always)]
    pub fn west(&self) -> Location {
        self.left()
    }

    /// Create a new location at one step in the given direction
    ///
    /// ```
    /// # use aoc23::Location;
    /// # use aoc23::Direction;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.apply(Direction::Up), Location { row: 0, col: 1});
    /// ```
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

    /// Create a new location at `n` steps in the given direction
    ///
    /// ```
    /// # use aoc23::Location;
    /// # use aoc23::Direction;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.apply_n(Direction::Down, 3), Location { row: 4, col: 1});
    /// ```
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

    /// Reverse the direction.
    ///
    /// ```
    /// # use aoc23::Direction;
    /// assert_eq!(Direction::Up.rev(), Direction::Down);
    /// assert_eq!(Direction::Down.rev(), Direction::Up);
    /// assert_eq!(Direction::Left.rev(), Direction::Right);
    /// assert_eq!(Direction::Right.rev(), Direction::Left);
    /// ```
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
