use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Galaxy(usize),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Galaxy {
    id: usize,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct SparseImage(Vec<Galaxy>);

impl SparseImage {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut galaxy_count = 0_usize;
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
        let mut map = Vec::with_capacity(galaxy_count);
        for (row, v) in image.into_iter().enumerate() {
            for (col, p) in v.into_iter().enumerate() {
                match p {
                    Pixel::Empty => {}
                    Pixel::Galaxy(id) => map.push(Galaxy { id, row, col }),
                }
            }
        }
        SparseImage(map)
    }

    fn galaxies(&self) -> impl Iterator<Item = &Galaxy> {
        self.0.iter()
    }

    fn col_empty(&self, col: usize) -> bool {
        self.galaxies().all(|g| g.col != col)
    }

    fn row_empty(&self, row: usize) -> bool {
        self.galaxies().all(|g| g.row != row)
    }

    fn expand(&self, factor: usize) -> Self {
        let factor = factor - 1;
        let map = self
            .galaxies()
            .copied()
            .map(|g| {
                let extra_rows = (0..g.row).filter(|row| self.row_empty(*row)).count() * factor;
                let extra_cols = (0..g.col).filter(|col| self.col_empty(*col)).count() * factor;
                Galaxy {
                    id: g.id,
                    row: g.row + extra_rows,
                    col: g.col + extra_cols,
                }
            })
            .collect_vec();
        SparseImage(map)
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let image = SparseImage::from_lines(lines);
    println!("{:?}", image);

    let new_image = image.expand(1000000);
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
