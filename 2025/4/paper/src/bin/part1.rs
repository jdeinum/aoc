use anyhow::{Context, Result};

type PaperGrid = Vec<Vec<bool>>;

struct PaddedPaperGrid {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl PaddedPaperGrid {
    /// no bounds checking for performance, should probably use bounds checks lol
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> bool {
        self.data[row * self.width + col]
    }

    pub fn build(paper_grid: PaperGrid) -> Result<Self> {
        let orig_height = paper_grid.len();
        let orig_width = paper_grid
            .get(0)
            .ok_or(anyhow::anyhow!("No rows in grid"))?
            .len();

        let width = orig_width + 2; // +2 for left and right padding
        let height = orig_height + 2; // +2 for top and bottom padding

        let mut data = vec![false; width * height];

        for (i, row) in paper_grid.into_iter().enumerate() {
            let dest_row = i + 1;
            let start_idx = dest_row * width + 1;
            for (j, val) in row.into_iter().enumerate() {
                data[start_idx + j] = val;
            }
        }

        Ok(Self {
            data,
            width,
            height,
        })
    }

    pub fn get_accessible(&self) -> Result<Vec<(usize, usize)>> {
        // result set
        let mut res: Vec<(usize, usize)> = Vec::new();

        for row in 1..(self.height - 1) {
            for col in 1..(self.width - 1) {
                // if not paper here, no need to consider
                if !self.get(row, col) {
                    continue;
                }

                // Count how many surrounding neighbors are paper
                // With flat vec, neighbor access is simple arithmetic
                let neighbor_count = [
                    self.get(row - 1, col - 1), // above-left
                    self.get(row - 1, col),     // above
                    self.get(row - 1, col + 1), // above-right
                    self.get(row, col - 1),     // left
                    self.get(row, col + 1),     // right
                    self.get(row + 1, col - 1), // below-left
                    self.get(row + 1, col),     // below
                    self.get(row + 1, col + 1), // below-right
                ]
                .iter()
                .filter(|&&paper| paper == true)
                .count();

                println!("neighbor count for ({row}, {col}) is {neighbor_count}");

                // atleast 4 paper neighbors
                if neighbor_count < 4 {
                    res.push((row, col));
                }
            }
        }

        Ok(res)
    }
}

fn parse_papers(grid_str: &str) -> Result<PaperGrid> {
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in grid_str.trim().split("\n") {
        let row = line
            .chars()
            .map(|x| match x {
                '@' => true,
                _ => false,
            })
            .collect::<Vec<bool>>();

        grid.push(row);
    }
    Ok(grid)
}

fn part1(grid_str: &str) -> Result<usize> {
    // convert grid into paper grid
    let grid = parse_papers(&grid_str).context("parse grid")?;

    // create a proper grid
    let padded_grid = PaddedPaperGrid::build(grid).context("build padded grid")?;

    // compute
    let positions = padded_grid
        .get_accessible()
        .context("get accessible papers")?;

    Ok(positions.len())
}

fn main() -> Result<()> {
    // read the input
    let f = std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("no input path provided"))?;

    let s = std::fs::read_to_string(f).context("read input file")?;

    let res = part1(&s).context("part 1")?;

    println!("res: {res}");

    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Context;

    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part1(input).context("part 1")?, 13);
        Ok(())
    }
}
