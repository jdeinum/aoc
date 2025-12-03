use anyhow::anyhow;
use itertools::Itertools;
use std::{collections::HashSet, io::Read};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow!("No path provided"))?;

    // open file
    let mut f = std::fs::File::open(&path).context("open path to file")?;

    // read all of the lines
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)
        .context("read bytes into string")?;

    let res = part2(&buf).context("run part 2")?;
    println!("res: {res}");

    Ok(())
}

fn part1(ranges: &str) -> Result<u64> {
    let id_ranges = parse_id_ranges(ranges).context("parse id ranges")?;
    let res = id_ranges
        .iter()
        .map(|x| x.find_invalid_part1())
        .flatten()
        .sum();
    Ok(res)
}
fn part2(ranges: &str) -> Result<u64> {
    let id_ranges = parse_id_ranges(ranges).context("parse id ranges")?;
    let res = id_ranges
        .iter()
        .map(|x| x.find_invalid_part2())
        .flatten()
        .collect::<HashSet<u64>>() // just incase there is overlapping ranges
        .iter()
        .sum();
    Ok(res)
}

fn parse_id_ranges(ranges: &str) -> Result<Vec<IdRange>> {
    let raw_ranges: Result<Vec<IdRange>> = ranges
        .trim()
        .split(",")
        .map(|x| IdRange::try_from(x))
        .collect();
    raw_ranges
}

struct IdRange {
    starting_number: u64,
    ending_number: u64,
}

impl TryFrom<&str> for IdRange {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (starting_number, ending_number) = value
            .split_once("-")
            .ok_or_else(|| anyhow::Error::msg("Range must contain -"))?;

        Ok(Self {
            starting_number: starting_number
                .parse()
                .with_context(|| format!("parse starting number: {starting_number}"))?,
            ending_number: ending_number
                .parse()
                .with_context(|| format!("parse ending_number number: {ending_number}"))?,
        })
    }
}

impl IdRange {
    pub fn find_invalid_part1(&self) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::new();

        // need end to be included
        for x in self.starting_number..self.ending_number + 1 {
            // obvious way is to convert to a string, but there is probably a better way
            // another small optimization here is to use a different allocator that handles similar
            // sized allocations well like slub
            let s = x.to_string();
            let len = s.len();

            // any odd number of characters can't be split into 2 of the same numbers
            if len % 2 == 1 {
                continue;
            }

            let (l, r) = s.split_at(len / 2);

            if l == r {
                println!("{l} == {r} so adding to set");
                res.push(x);
            }
        }

        res
    }

    pub fn find_invalid_part2(&self) -> HashSet<u64> {
        let mut res: HashSet<u64> = HashSet::new();

        // need end to be included
        for x in self.starting_number..self.ending_number + 1 {
            // obvious way is to convert to a string, but there is probably a better way
            // another small optimization here is to use a different allocator that handles similar
            // sized allocations well like slub
            let s = x.to_string();
            let len = s.len();

            let key_size = len / 2;

            for k in 1..=key_size {
                // if the length isn't a multiple of the key length, its not possible
                if len % k != 0 {
                    continue;
                }

                // break the string into len / key_size chunks
                // collecting into a string here is a little wasteful, but fine for the purpose
                if s.chars()
                    .chunks(k)
                    .into_iter()
                    .map(|x| x.collect::<String>())
                    .all_equal()
                {
                    println!("all chunks of size {k} equal for {x}");
                    res.insert(x);
                    break;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_example() -> Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
            446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let res = part1(&input).context("run part 1")?;

        assert_eq!(res, 1227775554);

        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
            446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let res = part2(&input).context("run part 1")?;

        assert_eq!(res, 4174379265);

        Ok(())
    }
}
