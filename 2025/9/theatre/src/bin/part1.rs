// The initial thought I have looking at this is that its some sort of computational geometry
// problem. The simplest solution I can think of is just to iterate through each point, and find
// the area of itself to each other point, being O(n^2).

use anyhow::{Context, Result};
use itertools::Itertools;

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

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    input.trim().split("\n").for_each(|x| {
        let (num1, num2) = x.split_once(",").unwrap();
        let num1 = num1.parse::<usize>().unwrap();
        let num2 = num2.parse::<usize>().unwrap();
        res.push((num1, num2));
    });
    res
}

fn part1(input: &str) -> Result<usize> {
    // parse the input
    let points = parse_input(input);

    println!("points: {points:?}");

    let res = points
        .iter()
        .tuple_combinations()
        .map(|(x, y)| (x.0.abs_diff(y.0) + 1) * (x.1.abs_diff(y.1) + 1))
        .max()
        .unwrap();

    Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Context;

    #[test]
    fn test_part1_example() -> Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part1(input).context("part 1")?, 50);
        Ok(())
    }
}
