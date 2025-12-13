// The structure I had for part 1 only needed small modifications in order to work. Rather than
// track just the current position, I tracked the entire path for debugging purposes, but this
// isn't actually necessary. Each Line in lines vec represents a unique path, and we just need to
// count the number that are in the finished state. Also we can no longer filter out non unique
// paths because even if they are at the same position, they may have been on different paths.
//
// While the test case passes, its far too slow for the primary input, which gets really unweildy
// around iteration 54. I actually need to model the problem in a better way in order for it to
// work. To me, this feels like a backtracking problem / dfs problem. We can model the input as a
// graph, where nodes are (row, col) positions where lines begin, while an edge between A and B
// represent a splitter connecting those positions. For example:
// .S.
// .^.
// ...
//
// Can be modeled as
// A(0,1) : start node
// B(1,0) : left line start after split
// C(1,2) : right line start after split
//
// A->B
// A->C
//
// Once we have our binary DAG (no cycles because we can't move backward in time), then all we need to do
// is find all unique paths through the graph using backtracking. Whenever we reach a node with no
// children, its a solution, and we append it to the set of solutions.
//
// Sike, purely DP, dont need the solutions themselves
use anyhow::{Context, Result};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Node {
    children: Option<(NodePointer, NodePointer)>,
}

type NodePointer = Rc<Node>;

pub struct Graph {
    start: NodePointer,
}

impl Graph {
    fn get_solution(&self) -> usize {
        let mut memo: HashMap<*const Node, usize> = HashMap::new();
        self.count_paths(&self.start, &mut memo)
    }

    fn count_paths(&self, node: &NodePointer, memo: &mut HashMap<*const Node, usize>) -> usize {
        let node_ptr = Rc::as_ptr(node);

        // Check if already computed
        if let Some(&count) = memo.get(&node_ptr) {
            return count;
        }

        // Compute count based on children
        let count = match &node.children {
            None => 1, // Leaf node = 1 path
            Some((left, right)) => {
                // Sum of children's path counts
                self.count_paths(left, memo) + self.count_paths(right, memo)
            }
        };

        // Cache result
        memo.insert(node_ptr, count);
        count
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn create_node(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    memo: &mut HashMap<(usize, usize), NodePointer>,
) -> NodePointer {
    // check if we've already created this node
    if let Some(existing) = memo.get(&(row, col)) {
        return existing.clone();
    }

    let children = find_children(row, col, grid, memo);
    let node = NodePointer::new(Node { children });

    // Cache the node before returning
    memo.insert((row, col), node.clone());
    node
}

// children are defined by running into a splitter
// Starting from (row, col), move down until hitting a splitter or exiting grid
fn find_children(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    memo: &mut HashMap<(usize, usize), NodePointer>,
) -> Option<(NodePointer, NodePointer)> {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    // Start from the next row down
    let mut current_row = row + 1;

    // Keep moving down until we hit a splitter or exit the grid
    while current_row < n_rows {
        if grid[current_row][col] == '^' {
            let mut children = Vec::new();

            // left child (col - 1)
            if col > 0 {
                let left_child = create_node(current_row, col - 1, grid, memo);
                children.push(left_child);
            }

            // right child (col + 1)
            if col + 1 < n_cols {
                let right_child = create_node(current_row, col + 1, grid, memo);
                children.push(right_child);
            }

            if children.len() == 2 {
                return Some((children[0].clone(), children[1].clone()));
            } else {
                return None;
            }
        }
        current_row += 1;
    }

    // exited the grid without hitting a splitter - this is a leaf node
    None
}

// construct the graph
fn parse_splitters_to_graph(input: &str) -> Graph {
    let grid = parse_grid(input);

    // find the start node (row 0, where 'S' is)
    let (start_row, start_col) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(c, _)| (r, c))
        })
        .unwrap();

    // create the start node (this will recursively build the entire graph)
    let mut memo = HashMap::new();
    let start = create_node(start_row, start_col, &grid, &mut memo);

    Graph { start }
}

fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read input")?;

    let res = part2(&s).context("part 1")?;

    println!("res: {res}");

    Ok(())
}

fn part2(input: &str) -> Result<usize> {
    let graph = parse_splitters_to_graph(input);
    let solutions = graph.get_solution();
    Ok(solutions)
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Context;

    #[test]
    fn test_part_2_example() -> Result<()> {
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
        assert_eq!(part2(input).context("part 1")?, 40);
        Ok(())
    }
}
