use std::collections::BTreeMap;
#[allow(dead_code)]
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn input_file() -> String {
    let input_file = std::env::args()
        .skip(1)
        .next()
        .expect("Expected input FILE");
    input_file
}

pub fn read_input_file() -> String {
    let input_file = input_file();
    read_to_string(input_file).expect("Could not read FILE")
}

pub fn read_input_lines() -> impl Iterator<Item = String> {
    let input_file = input_file();
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

pub fn read_visual_map(lines: impl Iterator<Item = String>) -> BTreeMap<Location, char> {
    let mut res = BTreeMap::new();
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            res.insert(Location::new_usize(row, col), c);
        }
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location {
    pub row: isize,
    pub col: isize,
}

impl Location {
    /// Create a new location at (row, col)
    ///
    /// ```
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
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
    /// # use aoc24::Location;
    /// let location = Location::new(1, 1);
    /// assert_eq!(location.west(), Location { row: 1, col: 0});
    /// ```
    #[inline(always)]
    pub fn west(&self) -> Location {
        self.left()
    }

    pub fn distance(&self, other: &Location) -> Distance {
        Distance {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }

    /// Create a new location at one step in the given direction
    ///
    /// ```
    /// # use aoc24::Location;
    /// # use aoc24::Direction;
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
    /// # use aoc24::Location;
    /// # use aoc24::Direction;
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

    /// Create a new location  in the given distance by vector addition
    ///
    /// ```
    /// # use aoc24::Location;
    /// # use aoc24::Distance;
    /// let location = Location::new(1, 1);
    /// let distance = Distance::new(-3, 1);
    /// assert_eq!(location.apply_distance(&distance), Location { row: -2, col: 2});
    #[inline(always)]
    pub fn apply_distance(&self, dis: &Distance) -> Self {
        Location {
            row: self.row + dis.row,
            col: self.col + dis.col,
        }
    }

    /// Create a new location in n times the given distance by vector addition
    ///
    /// ```
    /// # use aoc24::Location;
    /// # use aoc24::Distance;
    /// let location = Location::new(1, 1);
    /// let distance = Distance::new(-3, 1);
    /// assert_eq!(location.apply_n_distance(&distance, 2), Location { row: -5, col: 3});
    #[inline(always)]
    pub fn apply_n_distance(&self, dis: &Distance, n: isize) -> Self {
        Location {
            row: self.row + dis.row * n,
            col: self.col + dis.col * n,
        }
    }

    /// Check if the location is inside the bounding box formed by the rectangle
    /// that includes upper_left and lower_right as its corners (inclusive).
    ///
    /// # use aoc24::Location;
    /// # use aoc24::Distance;
    /// let location = Location::new(1, 1);
    /// assert!(location.is_inside_bounding_box(
    ///     &Location::new(0, 0), &Location::new(2, 2)));
    /// assert!(location.is_inside_bounding_box(
    ///     &Location::new(-1, -1), &Location::new(1, 1)));
    /// assert!(location.is_inside_bounding_box(
    ///     &Location::new(1, 1), &Location::new(2, 2)));
    #[inline(always)]
    pub fn is_inside_bounding_box(&self, upper_left: &Location, lower_right: &Location) -> bool {
        (upper_left.row..=lower_right.row).contains(&self.row)
            && (upper_left.col..=lower_right.col).contains(&self.col)
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
    /// # use aoc24::Direction;
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

impl Into<Distance> for Direction {
    fn into(self) -> Distance {
        use Direction::*;
        match self {
            Up => Distance::new(-1, 0),
            Down => Distance::new(1, 0),
            Right => Distance::new(0, 1),
            Left => Distance::new(0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Distance {
    pub row: isize,
    pub col: isize,
}

impl Distance {
    /// Create a new location at (row, col)
    ///
    /// ```
    /// # use aoc24::Distance;
    /// let distance = Distance::new(-1, 1);
    /// assert_eq!(distance, Distance { row: -1, col: 1});
    /// ```
    #[inline(always)]
    pub fn new(row: isize, col: isize) -> Self {
        Distance { row, col }
    }
}
