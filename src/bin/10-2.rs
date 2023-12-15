use anyhow;
use aoc23;
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
struct Location {
    row: isize,
    col: isize,
}

impl Location {
    fn north(&self) -> Location {
        Location {
            row: self.row - 1,
            col: self.col,
        }
    }
    fn south(&self) -> Location {
        Location {
            row: self.row + 1,
            col: self.col,
        }
    }
    fn east(&self) -> Location {
        Location {
            row: self.row,
            col: self.col + 1,
        }
    }
    fn west(&self) -> Location {
        Location {
            row: self.row,
            col: self.col - 1,
        }
    }
    fn add_row(&self, r: isize) -> Location {
        Location {
            row: self.row + r,
            col: self.col,
        }
    }
    fn add_col(&self, c: isize) -> Location {
        Location {
            row: self.row,
            col: self.col + c,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pipe {
    kind: PipeKind,
    con_1: Location,
    con_2: Location,
    loc: Location,
}

impl Pipe {
    fn swap_con(&self) -> Self {
        Pipe {
            kind: self.kind,
            con_1: self.con_2,
            con_2: self.con_1,
            loc: self.loc,
        }
    }

    fn has_con(&self, con: Location) -> bool {
        self.con_1 == con || self.con_2 == con
    }

    fn right_side_from_con(&self, con: Location) -> impl Iterator<Item = Location> {
        if !self.has_con(con) {
            return Vec::new().into_iter();
        }

        let pipe = if self.con_1 == con {
            *self
        } else {
            self.swap_con()
        };
        match pipe.kind {
            PipeKind::NorthToSouth => {
                //  1
                // X|
                //  2
                if pipe.con_1.row < pipe.con_2.row {
                    vec![pipe.loc.west()]
                //  2
                //  |X
                //  1
                } else {
                    vec![pipe.loc.east()]
                }
            }
            PipeKind::EastToWest => {
                //
                // 1-2
                //  X
                if pipe.con_1.col < pipe.con_2.col {
                    vec![pipe.loc.south()]
                //  X
                // 2-1
                //
                } else {
                    vec![pipe.loc.north()]
                }
            }
            PipeKind::NorthToEast => {
                //  1
                // XL2
                // XX
                if pipe.con_1.row < pipe.con_2.row {
                    vec![pipe.loc.west(), pipe.loc.west().south(), pipe.loc.south()]
                // 2X
                // L1
                } else {
                    vec![pipe.loc.north().east()]
                }
            }
            PipeKind::NorthToWest => {
                // X1
                // 2J
                if pipe.con_1.row < pipe.con_2.row {
                    vec![pipe.loc.north().west()]
                //  2
                // 1JX
                //  XX
                } else {
                    vec![pipe.loc.south(), pipe.loc.east().south(), pipe.loc.east()]
                }
            }
            PipeKind::SouthToEast => {
                // XX
                // XF1
                //  2
                if pipe.con_1.row < pipe.con_2.row {
                    vec![pipe.loc.north(), pipe.loc.north().west(), pipe.loc.west()]
                //  F2
                //  1X
                } else {
                    vec![pipe.loc.south().east()]
                }
            }
            PipeKind::SouthToWest => {
                // 17
                // X2
                if pipe.con_1.row < pipe.con_2.row {
                    vec![pipe.loc.south().west()]
                //  XX
                // 27X
                //  1
                } else {
                    vec![pipe.loc.east(), pipe.loc.north().east(), pipe.loc.north()]
                }
            }
        }
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char, row: usize, col: usize) -> Self {
        let row = row as isize;
        let col = col as isize;
        match c {
            '|' => Tile::Pipe(Pipe {
                kind: PipeKind::NorthToSouth,
                con_1: Location { row: row - 1, col },
                con_2: Location { row: row + 1, col },
                loc: Location { row, col },
            }),
            '-' => Tile::Pipe(Pipe {
                kind: PipeKind::EastToWest,
                con_1: Location { row, col: col - 1 },
                con_2: Location { row, col: col + 1 },
                loc: Location { row, col },
            }),
            'L' => Tile::Pipe(Pipe {
                kind: PipeKind::NorthToEast,
                con_1: Location { row: row - 1, col },
                con_2: Location { row, col: col + 1 },
                loc: Location { row, col },
            }),
            'J' => Tile::Pipe(Pipe {
                kind: PipeKind::NorthToWest,
                con_1: Location { row: row - 1, col },
                con_2: Location { row, col: col - 1 },
                loc: Location { row, col },
            }),
            '7' => Tile::Pipe(Pipe {
                kind: PipeKind::SouthToWest,
                con_1: Location { row: row + 1, col },
                con_2: Location { row, col: col - 1 },
                loc: Location { row, col },
            }),
            'F' => Tile::Pipe(Pipe {
                kind: PipeKind::SouthToEast,
                con_1: Location { row: row + 1, col },
                con_2: Location { row, col: col + 1 },
                loc: Location { row, col },
            }),
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

    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn area(&self) -> usize {
        self.size().0 * self.size().1
    }

    fn find_start_from(&self, mut loc: Location) -> Option<Vec<Location>> {
        let mut trail: Vec<Location> = Vec::new();
        trail.push(self.start_location());

        loop {
            // println!("search at {:?}", loc);
            match self.get(loc).copied() {
                None => return None,
                Some(Tile::Ground) => return None,
                Some(Tile::Start) => return Some(trail),
                Some(Tile::Pipe(Pipe {
                    kind: _,
                    con_1,
                    con_2,
                    loc: _pipe_loc,
                })) => {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FillTile {
    Filled,
    Unfilled(Tile),
}

impl FillTile {
    fn as_pipe(&self) -> Option<&Pipe> {
        match self {
            &FillTile::Unfilled(Tile::Pipe(ref p)) => Some(p),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct FillMap {
    // NOTE: All Unfilled(Tile::Pipe) belong to the trail
    map: Vec<Vec<FillTile>>,
}

impl FillMap {
    fn from_map_trail(map: Map, trail: Vec<Location>) -> Self {
        let map = map
            .0
            .into_iter()
            .enumerate()
            .map(|(row, v)| {
                v.into_iter()
                    .enumerate()
                    .map(|(col, tile)| {
                        if trail.contains(&Location {
                            row: row as isize,
                            col: col as isize,
                        }) {
                            FillTile::Unfilled(tile)
                        } else {
                            FillTile::Unfilled(Tile::Ground)
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        FillMap { map }
    }

    fn get(&self, loc: Location) -> Option<&FillTile> {
        if loc.row < 0 || loc.col < 0 {
            None
        } else {
            self.map
                .get(loc.row as usize)
                .and_then(|row| row.get(loc.col as usize))
        }
    }

    fn get_mut(&mut self, loc: Location) -> Option<&mut FillTile> {
        if loc.row < 0 || loc.col < 0 {
            None
        } else {
            self.map
                .get_mut(loc.row as usize)
                .and_then(|row| row.get_mut(loc.col as usize))
        }
    }

    fn flood_fill(&mut self, seed: Location) {
        let mut worklist: Vec<Location> = Vec::new();
        worklist.push(seed);
        while let Some(loc) = worklist.pop() {
            match self.get(loc).copied() {
                None => { /* continue */ }
                Some(FillTile::Filled) => { /* continue */ }
                Some(FillTile::Unfilled(Tile::Pipe(_))) => { /* continue hit wall of trace */ }
                Some(FillTile::Unfilled(Tile::Start)) => { /* continue hit wall of trace */ }
                Some(FillTile::Unfilled(Tile::Ground)) => {
                    *self.get_mut(loc).unwrap() = FillTile::Filled;
                    worklist.push(loc.north());
                    worklist.push(loc.south());
                    worklist.push(loc.east());
                    worklist.push(loc.west());
                }
            }
        }
    }

    fn print(&self) {
        for v in &self.map {
            for tile in v {
                match tile {
                    FillTile::Filled => print!("O"),
                    FillTile::Unfilled(Tile::Ground) => print!("."),
                    FillTile::Unfilled(Tile::Start) => print!("S"),
                    FillTile::Unfilled(Tile::Pipe(p)) => match p.kind {
                        PipeKind::NorthToSouth => print!("|"),
                        PipeKind::EastToWest => print!("-"),
                        PipeKind::NorthToEast => print!("L"),
                        PipeKind::NorthToWest => print!("J"),
                        PipeKind::SouthToEast => print!("F"),
                        PipeKind::SouthToWest => print!("7"),
                    },
                }
            }
            println!("");
        }
    }

    fn count_ground(&self) -> usize {
        self.map
            .iter()
            .flat_map(|v| v.iter())
            .filter(|&t| t == &FillTile::Unfilled(Tile::Ground))
            .count()
    }
    fn count_filled(&self) -> usize {
        self.map
            .iter()
            .flat_map(|v| v.iter())
            .filter(|&t| t == &FillTile::Filled)
            .count()
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
    println!("dist={}", dist);

    let trail_tiles_count = trail.len();
    let rest_tiles_count = map.area() - trail_tiles_count;
    dbg!(map.area());
    dbg!(trail_tiles_count);
    dbg!(rest_tiles_count);

    let mut fill_map = FillMap::from_map_trail(map, trail.clone());

    fill_map.print();

    for (last_loc, curr_loc) in trail.iter().copied().tuple_windows() {
        let curr = fill_map.get(curr_loc).unwrap().as_pipe().unwrap();
        for f_loc in curr.right_side_from_con(last_loc) {
            fill_map.flood_fill(f_loc);
        }
    }

    println!("##################\n###\n###\n");
    fill_map.print();

    // we dont know if we filled outside or inside the loop,
    // but one of them must be correct

    println!(
        "fill map ground count (left side, probably inside): {}",
        fill_map.count_ground()
    );
    println!(
        "fill map fill count (right side, probably outside): {}",
        fill_map.count_filled()
    );

    //println!("area={}", area);

    Ok(())
}
