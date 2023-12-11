use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Galaxy(u32),
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Image(Vec<Vec<Pixel>>);

impl Image {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut galaxy_count = 0;
        let image = lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Pixel::Galaxy({
                            galaxy_count += 1;
                            galaxy_count
                        }),
                        '.' => Pixel::Empty,
                        _ => panic!("Invalid image"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Image(image)
    }

    fn expand_rows(&self) -> Self {
        let mut image = Vec::new();
        for row in &self.0 {
            if row.iter().all_equal_value() == Ok(&Pixel::Empty) {
                image.push(row.clone());
                image.push(row.clone());
            } else {
                image.push(row.clone());
            }
        }
        Image(image)
    }

    fn expand_cols(&self) -> Self {
        let mut col_empty = Vec::new();
        for col_n in 0..self.0[0].len() {
            if self.0.iter().map(|row| row[col_n]).all_equal_value() == Ok(Pixel::Empty) {
                col_empty.push(true);
            } else {
                col_empty.push(false);
            }
        }
        let image = self
            .0
            .iter()
            .map(|row| {
                let mut new_row = Vec::new();
                for (col, pixel) in row.iter().copied().enumerate() {
                    if col_empty[col] {
                        new_row.push(pixel);
                        new_row.push(pixel);
                    } else {
                        new_row.push(pixel)
                    }
                }
                new_row
            })
            .collect_vec();
        Image(image)
    }

    fn galaxies(&self) -> impl Iterator<Item = Galaxy> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(row, v)| v.iter().enumerate().map(move |(col, p)| (row, col, p)))
            .filter_map(|(row, col, p)| match p {
                Pixel::Empty => None,
                Pixel::Galaxy(id) => Some(Galaxy { id: *id, row, col }),
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Galaxy {
    id: u32,
    row: usize,
    col: usize,
}

impl Galaxy {
    fn dist(&self, other: &Galaxy) -> usize {
        let dr = (self.row as isize - other.row as isize).abs() as usize;
        let dc = (self.col as isize - other.col as isize).abs() as usize;
        dr + dc
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let image = Image::from_lines(lines);
    println!("{:?}", image);
    let new_image = image.expand_rows().expand_cols();
    println!("{:?}", new_image);
    let galaxies = new_image.galaxies().collect_vec();
    println!("{:?}", galaxies);
    let sum: usize = galaxies
        .iter()
        .combinations(2)
        .map(|gg| gg[0].dist(gg[1]))
        .sum();
    println!("{}", sum);
    Ok(())
}
