use anyhow::{Context, Result};
use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};

fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read input")?;

    let res = part1(&s).context("part 1")?;

    println!("res: {res}");

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    // create the grid
    let grid = Grid::build(input);

    // create the line at the start
    let line = Line {
        x: grid.start.0,
        y: grid.start.1,
        status: LineStatus::Active,
    };

    let game = Game {
        grid,
        lines: vec![line],
        finished_lines: vec![],
    };

    game.play()
}

struct Game {
    pub grid: Grid,
    pub lines: Vec<Line>,
    pub finished_lines: Vec<Line>,
}

impl Game {
    fn play(mut self) -> Result<usize> {
        loop {
            // check if there are no lines active, all done
            if self.lines.is_empty() {
                break;
            }

            // advance all of the lines
            self.lines.iter_mut().for_each(|x| x.advance());

            // get the lines that are on a splitter or finished
            let finished: Vec<(usize, Line)> = self
                .lines
                .iter()
                .enumerate()
                .filter(|(_, x)| x.on_splitter(&self.grid) || x.finished(&self.grid))
                .map(|(a, b)| {
                    (
                        a,
                        Line {
                            status: match b.on_splitter(&self.grid) {
                                true => LineStatus::Split(b.x, b.y),
                                false => LineStatus::End(b.x, b.y),
                            },
                            ..b.clone()
                        },
                    )
                })
                .collect();

            // collect new lines to spawn from splitters
            let new_lines: Vec<Line> = finished
                .iter()
                .filter(|x| matches!(x.1.status, LineStatus::Split(_, _)))
                .flat_map(|(_, x)| {
                    let mut spawned = Vec::new();
                    // create new line to left
                    if x.y > 0 {
                        spawned.push(Line {
                            x: x.x,
                            y: x.y - 1,
                            status: LineStatus::Active,
                        });
                    }
                    // create new line to right
                    if x.y < self.grid.n_columns - 1 {
                        spawned.push(Line {
                            x: x.x,
                            y: x.y + 1,
                            status: LineStatus::Active,
                        });
                    }
                    spawned
                })
                .unique_by(|x| (x.x, x.y))
                .collect();

            // remove the lines on splitters or finished
            // deleting in reverse sorted sorted means we don't mess up indexes
            // the sorting should be pretty cheap because there aren't that many elements
            finished
                .iter()
                .sorted_by(|(i1, _), (i2, _)| i1.cmp(i2).reverse())
                .for_each(|(i, _)| {
                    self.lines.remove(*i);
                });

            // add the new lines to the lines
            self.lines.extend(new_lines);

            // add the split and finished lines to the finished
            self.finished_lines
                .extend(finished.into_iter().map(|(_, x)| x));
        }

        // was getting an extra split somehow, unsure why
        let unique_splits: HashSet<(usize, usize)> = self
            .finished_lines
            .iter()
            .filter_map(|line| match line.status {
                LineStatus::Split(x, y) => Some((x, y)),
                _ => None,
            })
            .collect();
        Ok(unique_splits.len())
    }
}

struct Grid {
    pub n_rows: usize,
    pub n_columns: usize,
    pub splitters: HashSet<(usize, usize)>,
    pub start: (usize, usize),
}

impl Grid {
    fn build(input: &str) -> Self {
        let rows_with_columns: Vec<(usize, Vec<(usize, char)>)> = input
            .split("\n")
            .enumerate()
            .map(|x| (x.0, x.1.chars().enumerate().collect::<Vec<(usize, char)>>()))
            .collect();
        let n_rows = rows_with_columns.len();
        let n_columns = rows_with_columns[0].1.len();
        let mut start: Option<(usize, usize)> = None;
        let mut splitters: HashSet<(usize, usize)> = HashSet::new();

        for (r_index, row) in rows_with_columns {
            for (c_index, c) in row {
                if c == '^' {
                    splitters.insert((r_index, c_index));
                }

                if c == 'S' {
                    start = Some((r_index, c_index));
                }
            }
        }

        Self {
            splitters,
            start: start.unwrap(),
            n_columns,
            n_rows,
        }
    }

    fn is_splitter(&self, x: usize, y: usize) -> bool {
        self.splitters.contains(&(x, y))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum LineStatus {
    Active,
    Split(usize, usize),
    End(usize, usize),
}

#[derive(Clone, Debug)]
struct Line {
    pub x: usize,
    pub y: usize,
    pub status: LineStatus,
}

// TODO: Is it better to store in a map of some kind? We iterate often so contiguous layout is
// good, but we want to avoid adding duplicate lines. Maybe B-Tree maps because we get uniqueness
impl Hash for Line {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Line {
    fn on_splitter(&self, grid: &Grid) -> bool {
        grid.is_splitter(self.x, self.y)
    }

    fn finished(&self, grid: &Grid) -> bool {
        self.x == grid.n_rows - 1
    }

    fn advance(&mut self) {
        if self.status == LineStatus::Active {
            self.x += 1
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Context;

    #[test]
    fn test_part_1_example() -> Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part1(input).context("part 1")?, 21);
        Ok(())
    }
}
