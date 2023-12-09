use std::collections::HashMap;

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Card {
    N2 = 2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid Card {}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hand(Vec<Card>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HandBid {
    hand: Hand,
    bid: u64,
}

impl HandBid {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let cards = parts
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(Card::from_char)
            .collect_vec();
        let bid = parts.next().unwrap().trim().parse().unwrap();
        HandBid {
            hand: Hand(cards),
            bid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

impl Hand {
    fn sort_key(&self) -> (HandType, Vec<Card>) {
        (self.hand_type(), self.0.clone())
    }

    fn hand_type(&self) -> HandType {
        let counts: HashMap<Card, usize> = self.0.iter().copied().counts();

        if counts
            .values()
            .filter(|count| **count == 5)
            .next()
            .is_some()
        {
            return HandType::FiveOfaKind;
        }
        if counts
            .values()
            .filter(|count| **count == 4)
            .next()
            .is_some()
        {
            return HandType::FourOfaKind;
        }
        if counts
            .values()
            .filter(|count| **count == 3)
            .next()
            .is_some()
            && counts
                .values()
                .filter(|count| **count == 2)
                .next()
                .is_some()
        {
            return HandType::FullHouse;
        }
        if counts
            .values()
            .filter(|count| **count == 3)
            .next()
            .is_some()
        {
            return HandType::ThreeOfaKind;
        }
        if counts.values().filter(|count| **count == 2).count() == 2 {
            return HandType::TwoPair;
        }
        if counts.values().filter(|count| **count == 2).count() == 1 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let mut hands = lines.map(|s| HandBid::from_line(&s)).collect_vec();
    hands.sort_by_key(|hand| hand.hand.sort_key());
    println!(
        "{:#?}",
        hands
            .iter()
            .map(|hand| (hand, hand.hand.hand_type()))
            .collect_vec()
    );
    let sum: u64 = hands
        .iter()
        .enumerate()
        .map(|(rank_0, hand)| (rank_0 as u64 + 1) * hand.bid)
        .sum();
    println!("{}", sum);
    Ok(())
}
