use anyhow;
use aoc23;
use itertools::*;
use ndarray::prelude::*;
use ndarray_linalg::{error::LinalgError, Solve};

#[derive(Debug, Clone)]
struct Hailstone {
    id: usize,
    pos: [i64; 3],
    delta: [i64; 3],
}

impl Hailstone {
    fn from_line(id: usize, line: &str) -> Hailstone {
        let (pos, delta) = line.split_once('@').unwrap();
        let (x, y, z) = pos
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let pos = [x, y, z];
        let (x, y, z) = delta
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let delta = [x, y, z];
        Hailstone { id, pos, delta }
    }
}

fn intersect2d(a: &Hailstone, b: &Hailstone) -> Result<Option<[f64; 2]>, LinalgError> {
    // |adx bdx|
    // |ady bdy|
    let a_: Array2<f64> = arr2(&[
        [a.delta[0] as f64, b.delta[0] as f64],
        [a.delta[1] as f64, b.delta[1] as f64],
    ]);
    // |bx - ax|
    // |by - ay|
    let b: Array1<f64> = arr1(&[
        b.pos[0] as f64 - a.pos[0] as f64,
        b.pos[1] as f64 - a.pos[1] as f64,
    ]);
    let t = a_.solve_into(b)?;
    let t_a = t[0];
    let t_b = -t[1];
    if t_a < 0.0 || t_b < 0.0 {
        return Ok(None);
    }
    let intersection = [
        a.pos[0] as f64 + t_a * a.delta[0] as f64,
        a.pos[1] as f64 + t_a * a.delta[1] as f64,
    ];
    Ok(Some(intersection))
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let stones = lines
        .enumerate()
        .map(|(id, s)| Hailstone::from_line(id, &s))
        .collect_vec();

    let min_intersect_pos = 200_000_000_000_000.0f64;
    let max_intersect_pos = 400_000_000_000_000.0f64;
    // let min_intersect_pos = 7.0f64;
    // let max_intersect_pos = 27.0f64;

    let mut count = 0;
    for (a, b) in stones
        .iter()
        .cartesian_product(stones.iter())
        .filter(|(a, b)| a.id != b.id)
    {
        match intersect2d(a, b) {
            Err(e) => println!("{} vs {}: Error {}", a.id, b.id, e),
            Ok(None) => println!("{} vs {}: Intersection in the past", a.id, b.id),
            Ok(Some(v)) => {
                println!("{} vs {}: Intersect at {:?}", a.id, b.id, v);
                if v[0] >= min_intersect_pos
                    && v[0] <= max_intersect_pos
                    && v[1] >= min_intersect_pos
                    && v[1] <= max_intersect_pos
                {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count / 2);
    Ok(())
}
