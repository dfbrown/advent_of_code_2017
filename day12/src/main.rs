use std::fs::File;
use std::io::prelude::*;
use std::collections::vec_deque;

// Get the input graph as an adjacency list
fn get_input() -> Vec<Vec<usize>> {
    let mut f = File::open("input.txt").expect("Could not open file");
    let mut input_str = String::new();
    f.read_to_string(&mut input_str)
        .expect("Could not read file");

    let mut result: Vec<Vec<usize>> = Vec::new();

    for (i, line) in input_str.lines().enumerate() {
        let mut numbers_iter = line.split(|c: char| !c.is_digit(10))
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<usize>().expect("Not a number??"));
        let first_number = numbers_iter.next().expect("No first number?");
        assert!(first_number == i, "Expected index {}, got {}", i, first_number);

        result.push(numbers_iter.collect());
    }

    return result;
}

// Returns a vector where each element is the size of a group in the graph.  The first entry will
// be the group that contains Node 0 (so the answer to part 1)
fn get_group_sizes(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut seen_node = vec![false; graph.len()];
    let mut next_nodes = vec_deque::VecDeque::with_capacity(10);
    let mut group_sizes = Vec::new();
    while let Some((i, &_)) = seen_node.iter().enumerate().find(|&(_, &x)| !x) {
        assert!(next_nodes.len() == 0);
        next_nodes.push_back(i);
        seen_node[i] = true;
        let mut group_size = 1;
        while next_nodes.len() > 0 {
            let node_index = next_nodes.pop_front().unwrap();
            for &child in graph[node_index].iter() {
                if !seen_node[child] {
                    seen_node[child] = true;
                    next_nodes.push_back(child);
                    group_size += 1
                }
            }
        }
        group_sizes.push(group_size);
    }

    return group_sizes;
}

fn main() {
    let input_graph = get_input();
    let group_sizes = get_group_sizes(&input_graph);
    println!("Part 1: {}", group_sizes[0]);
    println!("Part 2: {}", group_sizes.len());
}
