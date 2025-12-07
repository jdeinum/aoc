use anyhow::{Context, Result};
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::digit1, sequence::separated_pair,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn parse_number(input: &str) -> IResult<&str, u64> {
    digit1.map_res(|s: &str| s.parse::<u64>()).parse(input)
}

// parses x-y
fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(parse_number, tag("-"), parse_number)
        .map(|(start, end)| start..=end)
        .parse(input)
}

// Parse from string (for tests)
fn parse_data(input: &str) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let mut lines = input.lines();

    // Parse ranges section
    let mut ranges = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break; // Hit the blank line separator
        }
        let (_, range) =
            parse_range(line).map_err(|e| anyhow::anyhow!("Parse range error: {}", e))?;
        ranges.push(range);
    }

    // Parse queries section
    let mut queries = Vec::new();
    for line in lines {
        let (_, num) =
            parse_number(line).map_err(|e| anyhow::anyhow!("Parse number error: {}", e))?;
        queries.push(num);
    }

    Ok((ranges, queries))
}

// Parse file line by line for memory efficiency
fn parse_file_streaming(path: &str) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let file = File::open(path).context("open file")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse ranges section
    let mut ranges = Vec::new();
    for line in &mut lines {
        let line = line.context("read line")?;
        if line.trim().is_empty() {
            break; // Hit the blank line separator
        }
        let (_, range) =
            parse_range(&line).map_err(|e| anyhow::anyhow!("Parse range error: {}", e))?;
        ranges.push(range);
    }

    // Parse queries section
    let mut queries = Vec::new();
    for line in lines {
        let line = line.context("read line")?;
        let (_, num) =
            parse_number(&line).map_err(|e| anyhow::anyhow!("Parse number error: {}", e))?;
        queries.push(num);
    }

    Ok((ranges, queries))
}

struct RangeSet {
    ranges: Vec<RangeInclusive<u64>>,
}

impl RangeSet {
    pub fn build(ranges: Vec<RangeInclusive<u64>>) -> Self {
        Self { ranges }
    }

    pub fn present(&self, values: &[u64]) -> Vec<u64> {
        values
            .iter()
            .filter(|&&v| self.ranges.iter().any(|range| range.contains(&v)))
            .copied()
            .collect()
    }
}

fn part1(input_data: &str) -> Result<usize> {
    let (ranges, queries) = parse_data(input_data).context("parse input data")?;

    let range_set = RangeSet::build(ranges);

    let present = range_set.present(&queries);

    Ok(present.len())
}

fn main() -> Result<()> {
    // get path to input
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    // Parse file line by line for memory efficiency
    let (ranges, queries) = parse_file_streaming(&fp).context("parse file")?;

    let range_set = RangeSet::build(ranges);
    let present = range_set.present(&queries);

    println!("res: {}", present.len());

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1_example() -> Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part1(input).context("part 1")?, 3);
        Ok(())
    }
}
