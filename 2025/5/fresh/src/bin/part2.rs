// Part 2 is interesting because it's more difficult to find non overlapping ranges than it is to
// find membership of the set. Once we find non overlapping ranges, it becomes pretty easy to count
// the number of unique elements, so I will focus on that.
//
// Some initial thoughts are as follows:
// 1. Each time we insert a range X, we go through the existing ranges and determine if X has any overlap with each of the existing ranges.
// For each range that has overlap, we extend those ranges to include the values they are missing
// from X
// If no, we insert X
// Once all of the ranges are exhausted, we flatten the non overlapping ranges and get a count
// without needing to materialize any of the values.
//
// When comparing two ranges X and Y, we have the following possibilites:
// 1. X << Y (x is a subset of y)
// 2. X >> Y (y is a subset of x)
// 3. Y.start < X.start < Y.end
// 4. X.start < Y.start < X.end
//
// In cases 1 or 2, we can keep only the superset, while in 3 and 4, we can add new ranges to add
// the missing values.
//
// ----------------------------------------------------------------------------------------------
//
// Or, instead of that, iterate through a series of rounds, each round, we determine whether any
// other range overlaps with ours. We take all of these ranges, and produce a new range that has
// the smallest start point and the furthest endpoint. We then move on to the next range. We have
// to continue iterating until our set doesn't change and thats our final non overlapping set.
//
// ^^ this is what i did

use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::digit1, sequence::separated_pair,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct UnbuiltRange {
    pub start: u64,
    pub end: u64,
}

impl UnbuiltRange {
    /// two ranges A,B overlap if either the start or end of B is within the bounds of A (including
    /// the edges) Assume we are B for diagrams.
    fn overlaps_uni_direction(&self, other: &Self) -> bool {
        // B1      A1    A2      B2
        // |       |     |       |
        // 1 2 3 4 5 6 7 8 9 10 11 12
        if self.start <= other.start && self.end >= other.end {
            return true;
        }
        // A1      B1    A2      B2
        // |       |     |       |
        // 1 2 3 4 5 6 7 8 9 10 11 12
        else if self.start >= other.start && self.start <= other.end {
            return true;
        }
        // B1      A1    B2      A2
        // |       |     |       |
        // 1 2 3 4 5 6 7 8 9 10 11 12
        else if self.end >= other.start && self.end <= other.end {
            return true;
        }
        // no overlap
        else {
            return false;
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.overlaps_uni_direction(other) || other.overlaps_uni_direction(self)
    }

    pub fn combine(&self, others: &[&UnbuiltRange]) -> Self {
        println!("combining {others:?}");

        // find the min start
        let min_start = [self].iter().chain(others).map(|x| x.start).min().unwrap();

        // find the max end
        let max_end = [self].iter().chain(others).map(|x| x.end).max().unwrap();

        // return the new range
        Self {
            start: min_start,
            end: max_end,
        }
    }
}

impl From<UnbuiltRange> for RangeInclusive<u64> {
    fn from(value: UnbuiltRange) -> Self {
        value.start..=value.end
    }
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    digit1.map_res(|s: &str| s.parse::<u64>()).parse(input)
}

// parses x-y
fn parse_range(input: &str) -> IResult<&str, UnbuiltRange> {
    separated_pair(parse_number, tag("-"), parse_number)
        .map(|(start, end)| UnbuiltRange { start, end })
        .parse(input)
}

// Parse from string (for tests)
fn parse_data(input: &str) -> Result<Vec<UnbuiltRange>> {
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

    Ok(ranges)
}

// Parse file line by line for memory efficiency
fn parse_file_streaming(path: &str) -> Result<Vec<UnbuiltRange>> {
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
    Ok(ranges)
}

#[derive(Clone, Debug)]
struct RangeSet {
    ranges: Vec<UnbuiltRange>,
}

impl RangeSet {
    pub fn build(ranges: Vec<UnbuiltRange>) -> Self {
        Self { ranges }
    }

    /// deduplicating consists of several rounds, where each round outputs the input for the
    /// previous round
    pub fn deduplicate_ranges(&mut self) -> Result<()> {
        let mut res: Vec<UnbuiltRange> = Vec::new();
        loop {
            let mut is_changed: bool = false;
            res.drain(..);

            for range in &self.ranges {
                println!("deduplicating {range:? }");
                // find overlaps
                let ranges_copy = self.ranges.clone();
                let overlapping_ranges: Vec<&UnbuiltRange> = ranges_copy
                    .iter()
                    .filter(|&x| x != range)
                    .filter(|x| x.overlaps(range))
                    .collect();

                if overlapping_ranges.is_empty() {
                    println!("no overlaps found for {range:?}");
                    res.push(range.clone());
                    continue;
                }

                println!("overlaps found for {range:?}: {overlapping_ranges:?}");
                // if we have any overlaps, we will be changing the result set
                is_changed = true;

                // combine them
                let combined = range.combine(&overlapping_ranges);
                println!("combined: {combined:?}");

                // add the new unified set
                res.push(combined);
            }

            println!("\n\n\n");

            self.ranges = res.iter().unique().cloned().collect();
            if !is_changed {
                break Ok(());
            }
        }
    }

    pub fn num_present(&self) -> usize {
        self.ranges
            .clone()
            .into_iter()
            .map(|x| std::convert::Into::<RangeInclusive<u64>>::into(x))
            .flatten()
            .count()
    }
}

fn part2(input_data: &str) -> Result<usize> {
    let ranges = parse_data(input_data).context("parse input data")?;

    let mut range_set = RangeSet::build(ranges);

    range_set
        .deduplicate_ranges()
        .context("deduplicate ranges")?;

    let res = range_set.num_present();

    Ok(res)
}

fn main() -> Result<()> {
    // get path to input
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    // Parse file line by line for memory efficiency
    let ranges = parse_file_streaming(&fp).context("parse file")?;

    let mut range_set = RangeSet::build(ranges);

    range_set
        .deduplicate_ranges()
        .context("deduplicate ranges")?;

    let num_present = range_set.num_present();

    println!("res: {}", num_present);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_2_example() -> Result<()> {
        let input = "3-5
10-14
16-20
12-18";
        assert_eq!(part2(input).context("part 2")?, 14);
        Ok(())
    }
}
