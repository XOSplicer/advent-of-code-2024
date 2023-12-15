use std::collections::HashSet;

use anyhow;
use aoc23;

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let mut parts = line.split(':');
        let id: u32 = parts
            .next()
            .unwrap()
            .strip_prefix("Card")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let mut number_parts = parts.next().unwrap().split('|');
        let winning_numbers: HashSet<u32> = number_parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let card_numbers: HashSet<u32> = number_parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Card {
            id,
            winning_numbers,
            card_numbers,
        }
    }

    fn card_winning_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .copied()
    }

    fn point_value(&self) -> u32 {
        let l = self.card_winning_numbers().count();
        if l == 0 {
            0
        } else {
            1 << (l - 1)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let sum: u32 = lines.map(|s| Card::from_line(&s).point_value()).sum();
    println!("{}", sum);
    Ok(())
}
