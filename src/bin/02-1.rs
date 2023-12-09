use std::str::FromStr;

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone)]
struct RevealedSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl RevealedSet {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    revealed_sets: Vec<RevealedSet>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.revealed_sets.iter().all(|r| r.is_valid())
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let id: u32 = parts
            .next()
            .ok_or(anyhow::anyhow!("Game id not found"))?
            .strip_prefix("Game")
            .ok_or(anyhow::anyhow!("Game id not found"))?
            .trim()
            .parse()?;
        let revealed_sets: Vec<RevealedSet> = parts
            .next()
            .ok_or(anyhow::anyhow!("Game sets not found"))?
            .split(';')
            .map(|r| r.trim())
            .map(|r| r.parse().unwrap())
            .collect_vec();
        Ok(Game { id, revealed_sets })
    }
}

impl FromStr for RevealedSet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').map(|p| p.trim());
        let red: u32 = parts
            .clone()
            .find(|p| p.contains("red"))
            .unwrap_or("0 red")
            .strip_suffix("red")
            .unwrap()
            .trim()
            .parse()?;
        let green: u32 = parts
            .clone()
            .find(|p| p.contains("green"))
            .unwrap_or("0 green")
            .strip_suffix("green")
            .unwrap()
            .trim()
            .parse()?;
        let blue: u32 = parts
            .clone()
            .find(|p| p.contains("blue"))
            .unwrap_or("0 blue")
            .strip_suffix("blue")
            .unwrap()
            .trim()
            .parse()?;
        Ok(RevealedSet { red, green, blue })
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let sum: u32 = lines
        .map(|s| s.parse::<Game>().unwrap())
        .filter(|g| g.is_valid())
        .map(|g| g.id)
        .sum();
    println!("{}", sum);
    Ok(())
}
