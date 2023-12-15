use anyhow;
use aoc23;

fn parse_digit(s: &str) -> Option<u32> {
    if s.starts_with("one") || s.starts_with("1") {
        Some(1)
    } else if s.starts_with("two") || s.starts_with("2") {
        Some(2)
    } else if s.starts_with("three") || s.starts_with("3") {
        Some(3)
    } else if s.starts_with("four") || s.starts_with("4") {
        Some(4)
    } else if s.starts_with("five") || s.starts_with("5") {
        Some(5)
    } else if s.starts_with("six") || s.starts_with("6") {
        Some(6)
    } else if s.starts_with("seven") || s.starts_with("7") {
        Some(7)
    } else if s.starts_with("eight") || s.starts_with("8") {
        Some(8)
    } else if s.starts_with("nine") || s.starts_with("9") {
        Some(9)
    } else if s.starts_with("0") {
        Some(0)
    } else {
        None
    }
}

static NEEDLE: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
    "4", "5", "6", "7", "8", "9",
];

fn find_first(s: &str) -> Option<usize> {
    NEEDLE.iter().filter_map(|n| s.find(n)).min()
}

fn find_last(s: &str) -> Option<usize> {
    NEEDLE.iter().filter_map(|n| s.rfind(n)).max()
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();

    let sum: u32 = lines
        .map(|s| {
            let first = find_first(&s)
                .map(|m| parse_digit(&s[m..]).unwrap())
                .unwrap_or(0);
            let last = find_last(&s)
                .map(|m| parse_digit(&s[m..]).unwrap())
                .unwrap_or(0);
            first * 10 + last
        })
        .sum();
    println!("{}", sum);
    Ok(())
}
