use anyhow;
use aoc23;
use aoc23::Location;
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeKind {
    /// Vertical
    NorthToSouth,
    /// Horizontal
    EastToWest,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Pipe {
        kind: PipeKind,
        con_1: Location,
        con_2: Location,
    },
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char, row: usize, col: usize) -> Self {
        let row = row as isize;
        let col = col as isize;
        match c {
            '|' => Tile::Pipe {
                kind: PipeKind::NorthToSouth,
                con_1: Location { row: row - 1, col },
                con_2: Location { row: row + 1, col },
            },
            '-' => Tile::Pipe {
                kind: PipeKind::EastToWest,
                con_1: Location { row, col: col - 1 },
                con_2: Location { row, col: col + 1 },
            },
            'L' => Tile::Pipe {
                kind: PipeKind::NorthToEast,
                con_1: Location { row: row - 1, col },
                con_2: Location { row, col: col + 1 },
            },
            'J' => Tile::Pipe {
                kind: PipeKind::NorthToWest,
                con_1: Location { row: row - 1, col },
                con_2: Location { row, col: col - 1 },
            },
            '7' => Tile::Pipe {
                kind: PipeKind::SouthToWest,
                con_1: Location { row: row + 1, col },
                con_2: Location { row, col: col - 1 },
            },
            'F' => Tile::Pipe {
                kind: PipeKind::SouthToEast,
                con_1: Location { row: row + 1, col },
                con_2: Location { row, col: col + 1 },
            },
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Unknown Tile: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let map = lines
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| Tile::from_char(c, row, col))
                    .collect_vec()
            })
            .collect_vec();
        Map(map)
    }

    fn start_location(&self) -> Location {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(row, v)| {
                v.iter()
                    .enumerate()
                    .map(move |(col, tile)| (row, col, tile))
            })
            .find(|(_row, _col, &tile)| tile == Tile::Start)
            .map(|(row, col, _tile)| Location {
                row: row as isize,
                col: col as isize,
            })
            .unwrap()
    }

    fn get(&self, loc: Location) -> Option<&Tile> {
        if loc.row < 0 || loc.col < 0 {
            None
        } else {
            self.0
                .get(loc.row as usize)
                .and_then(|row| row.get(loc.col as usize))
        }
    }

    fn find_start_from(&self, mut loc: Location) -> Option<Vec<Location>> {
        let mut trail: Vec<Location> = Vec::new();
        trail.push(self.start_location());

        loop {
            println!("search at {:?}", loc);
            match self.get(loc).copied() {
                None => return None,
                Some(Tile::Ground) => return None,
                Some(Tile::Start) => return Some(trail),
                Some(Tile::Pipe {
                    kind: _,
                    con_1,
                    con_2,
                }) => {
                    let last_loc = *trail.last().unwrap();
                    if con_1 == last_loc {
                        trail.push(loc);
                        loc = con_2;
                    } else if con_2 == last_loc {
                        trail.push(loc);
                        loc = con_1;
                    } else {
                        println!("Inconsistent path: Stuck at {:?}", loc);
                        return None;
                    }
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let map = Map::from_lines(lines);
    // println!("{:?}", &map);
    let start = map.start_location();
    println!("Start: {:?}", &start);
    let trail = [start.north(), start.south(), start.east(), start.west()]
        .into_iter()
        .find_map(|search_loc| map.find_start_from(search_loc))
        .unwrap();
    println!("Trail: {:?}", &trail);
    let dist = trail.len() / 2;
    println!("{}", dist);
    Ok(())
}
