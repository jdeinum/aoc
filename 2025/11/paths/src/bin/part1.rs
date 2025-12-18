// At first glance, this seems like a backtracking solution, but we don't actually need the paths
// themselves for anything, so we can reduce this to a DP problem where we just keep track of the
// number of ways each node has to the out tag.
//
// We can do this using a topological sort using DFS, which will ensure that when we get to Node X,
// we know the number of paths from node X to the exit.
use anyhow::{Context, Result};
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    // parse the graph
    let graph = parse_graph(input);
    count_paths(&graph, "you")
}

fn count_paths(graph: &HashMap<String, Vec<String>>, start: &str) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    count_paths_inner(graph, start, &mut cache)
}

fn count_paths_inner(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    // if already in the cache, use that
    if let Some(count) = cache.get(current) {
        return *count;
    }

    // if its the end, we are at our destination, and there is only 1 way to get there
    if current == "out" {
        return 1;
    }

    // if not at the end, we need to get the sum of our neighbors
    graph
        .get(current)
        .unwrap()
        .iter()
        .map(|x| count_paths_inner(graph, x, cache))
        .sum()
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
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(part1(input), 5);
        Ok(())
    }
}
