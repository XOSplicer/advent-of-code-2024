use anyhow;
use aoc24;

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();
    let map = aoc24::read_visual_map_filter_map(lines, |c: char| match c {
        '.' => None,
        _ => Some(c.to_digit(10).unwrap()),
    });

    dbg!(&map);

    let res: u32 = 0;
    println!("{}", res);

    Ok(())
}
