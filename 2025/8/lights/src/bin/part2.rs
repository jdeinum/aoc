// Lights !
//
// This is just kruskals algorithm, where every node has an edge to every other node
//
// The first thing we do is build the edge graph, followed by running the algorithm itself

use anyhow::{Context, Result};
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Node {
    x: u64,
    y: u64,
    z: u64,
}

impl Node {
    fn distance_squared(&self, other: &Node) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);

        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Ord, Eq, Debug)]
struct Edge {
    start: usize,
    end: usize,
    cost: u64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).map(|x| x.reverse()) // reverse for min heap
    }
}

fn parse_input_to_nodes(input: &str) -> Vec<Node> {
    let mut res: Vec<Node> = Vec::new();
    let lines = input.trim().split("\n");

    for line in lines {
        let mut digits = line.split(",");
        let x = digits.next().unwrap().parse::<u64>().unwrap();
        let y = digits.next().unwrap().parse::<u64>().unwrap();
        let z = digits.next().unwrap().parse::<u64>().unwrap();

        res.push(Node { x, y, z });
    }

    res
}

fn get_edges(nodes: &[Node]) -> Vec<Edge> {
    let mut res: Vec<Edge> = Vec::new();
    nodes.iter().enumerate().for_each(|(start_index, start)| {
        nodes
            .iter()
            .enumerate()
            .skip(start_index + 1) // Skip self-loops
            .for_each(|(end_index, end)| {
                res.push(Edge {
                    start: start_index,
                    end: end_index,
                    cost: start.distance_squared(end),
                })
            })
    });

    res
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // already in same set
        }

        // Union by size: attach smaller to larger
        if self.size[root_x] >= self.size[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        }

        true // sets were merged
    }

    pub fn same_component(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn set_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

fn part2(input: &str) -> u64 {
    // get nodes
    let nodes = parse_input_to_nodes(input);

    // get the edges
    let mut edges: BinaryHeap<Edge> = get_edges(&nodes).into();

    // union find set
    let mut uf: UnionFind = UnionFind::new(nodes.len());

    // keep track of the nodes we are connecting
    let mut res: Vec<(usize, usize)> = Vec::new();

    // run the algorithm
    while let Some(edge) = edges.pop() {
        // check if they are already in the same component
        if uf.same_component(edge.start, edge.end) {
            continue;
        }

        // if not, make them part of the same component
        uf.union(edge.start, edge.end);

        // keep track of the order of node insertion
        res.push((edge.start, edge.end));
    }

    // get the nodes we connect last
    let (start, end) = res.last().unwrap();
    let diff = nodes.get(*start).unwrap().x * nodes.get(*end).unwrap().x;
    diff
}

fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read input")?;

    let res = part2(&s);

    println!("res: {res}");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part2_example() -> Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part2(input), 25272);
        Ok(())
    }
}
