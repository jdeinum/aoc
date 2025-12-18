// Similar to part 1, we can use DP with memoization, but we need to track additional state:
// whether we've visited "dac" and "fft" along the path. This allows us to count only the
// paths that visit both required nodes without enumerating all paths explicitly.

use anyhow::{Context, Result};
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    // parse the graph
    let graph = parse_graph(input);
    count_paths(&graph, "svr")
}

fn count_paths(graph: &HashMap<String, Vec<String>>, start: &str) -> usize {
    let mut cache: HashMap<(String, bool, bool), usize> = HashMap::new();
    count_paths_inner(graph, start, false, false, &mut cache)
}

fn count_paths_inner(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    visited_dac: bool,
    visited_fft: bool,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    // Update visited flags based on current node
    let visited_dac = visited_dac || current == "dac";
    let visited_fft = visited_fft || current == "fft";

    // Check cache
    let key = (current.to_string(), visited_dac, visited_fft);
    if let Some(&count) = cache.get(&key) {
        return count;
    }

    // If we reached "out", check if we've visited both required nodes
    if current == "out" {
        return if visited_dac && visited_fft { 1 } else { 0 };
    }

    // Recurse to neighbors and sum the path counts
    let count: usize = graph
        .get(current)
        .unwrap()
        .iter()
        .map(|next| count_paths_inner(graph, next, visited_dac, visited_fft, cache))
        .sum();

    cache.insert(key, count);
    count
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut res: HashMap<String, Vec<String>> = HashMap::new();

    // when processing a line, we may not have entries for its neighbors yet
    for line in input.trim().split("\n") {
        let mut values = line.split(" ");
        let id = values.next().unwrap().strip_suffix(":").unwrap();
        let values: Vec<&str> = values.collect();

        res.insert(
            id.to_string(),
            values.iter().map(|x| x.to_string()).collect(),
        );
    }

    res
}

fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read input")?;

    let res = part1(&s);

    println!("res: {res}");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_example() -> Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(part1(input), 2);
        Ok(())
    }
}
