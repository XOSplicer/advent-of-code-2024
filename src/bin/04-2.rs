use std::collections::HashSet;

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone)]
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
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let originals = lines.map(|s| Card::from_line(&s)).collect_vec();

    // the worklist algorithm is inefficient but it works
    let mut worklist = originals.iter().collect_vec();
    let mut count = 0_u32;
    while let Some(card) = worklist.pop() {
        count += 1;
        let w = card.card_winning_numbers().count();
        // println!("count={} id={} w={}", count, card.id, w);
        for i in 0..w {
            worklist.push(&originals[card.id as usize + i]);
        }
    }

    println!("{}", count);
    Ok(())
}
