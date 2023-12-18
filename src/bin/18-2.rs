use anyhow;
use aoc23::{self, Direction, Location};
use geo::{self, Area, Coord, LineString, Polygon};
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let mut parts = line.trim().split_whitespace();
        parts.next(); // skip direction
        parts.next(); // skip steps

        let hex_str = parts
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(')');

        let (steps_str, dir_str) = hex_str.split_at(5);
        let steps = usize::from_str_radix(steps_str, 16).unwrap();
        let direction = match dir_str {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            s => panic!("invalid direction: {}", s),
        };

        Instruction { direction, steps }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let instructions = lines.map(|s| Instruction::from_line(&s)).collect_vec();

    let mut curr_loc = Location::new(0, 0);
    let mut points = Vec::with_capacity(instructions.len());
    points.push(curr_loc);
    for instr in &instructions {
        curr_loc = curr_loc.apply_n(instr.direction, instr.steps);
        points.push(curr_loc);
    }
    // close ring
    points.push(Location::new(0, 0));

    let poly = Polygon::new(
        LineString::new(
            points
                .into_iter()
                .map(|loc| (loc.row as f64, loc.col as f64))
                .map(|c| Coord::from(c))
                .collect_vec(),
        ),
        Vec::new(),
    );

    dbg!(instructions.len());
    dbg!(poly.exterior().lines().count());
    let outer_correction = poly
        .exterior()
        .lines()
        .map(|l| (l.dx() + l.dy()).abs())
        .sum::<f64>()
        / 2.0
        + 1.0;

    let inner_area = poly.unsigned_area();
    dbg!(inner_area);
    println!("{}", inner_area + outer_correction);

    Ok(())
}
