use std::{collections::BTreeMap, fs::File, io::read_to_string};

use anyhow;
use aoc24::{self};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instr {
    Do,
    Dont,
    Mul(u64, u64),
}

fn main() -> anyhow::Result<()> {
    let input = aoc24::read_input_file();
    // matches mul(123,4) and captures 123 and 4
    let re =
        Regex::new(r"mul\((?P<n1>\d{1,3}),(?P<n2>\d{1,3})\)|(?P<do>do\(\))|(?P<dont>don't\(\))")
            .unwrap();

    let instrs = re.captures_iter(&input).map(|cap| {
        if cap.name("do").is_some() {
            return Instr::Do;
        }
        if cap.name("dont").is_some() {
            return Instr::Dont;
        }
        let s1 = cap.name("n1").unwrap().as_str();
        let s2 = cap.name("n2").unwrap().as_str();
        let n1: u64 = s1.parse().unwrap();
        let n2: u64 = s2.parse().unwrap();
        Instr::Mul(n1, n2)
    });
    let mut res: u64 = 0;
    let mut enable = true;
    for instr in instrs {
        match instr {
            Instr::Do => enable = true,
            Instr::Dont => enable = false,
            Instr::Mul(n1, n2) => {
                if enable {
                    res += n1 * n2;
                }
            }
        }
    }
    println!("{}", res);
    Ok(())
}
