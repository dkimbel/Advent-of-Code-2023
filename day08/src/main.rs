use std::collections::HashMap;
use std::fs;

use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Option<Instruction> {
        match c {
            'R' => Some(Instruction::Right),
            'L' => Some(Instruction::Left),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct NextNodes {
    left: String,
    right: String,
}

fn main() {
    let mut nodes_to_next_nodes: HashMap<String, NextNodes> = HashMap::new();
    let re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();
    let file_content = fs::read_to_string("resources/input_1").unwrap();
    let split = file_content.split("\n\n").collect::<Vec<_>>();
    let instructions = split[0]
        .chars()
        .map(|c| Instruction::from_char(c).unwrap())
        .collect::<Vec<_>>();
    for line in split[1].lines() {
        let caps = re.captures(line).unwrap();
        nodes_to_next_nodes.insert(
            caps[1].to_string(),
            NextNodes {
                left: caps[2].to_string(),
                right: caps[3].to_string(),
            },
        );
    }

    let mut num_moves: usize = 0;
    let instructions_len = instructions.len();
    let mut curr_node = "AAA";
    while curr_node != "ZZZ" {
        let i = num_moves % instructions_len;
        let next_nodes = nodes_to_next_nodes.get(curr_node).unwrap();
        let instruction = instructions[i];
        curr_node = match instruction {
            Instruction::Left => &next_nodes.left,
            Instruction::Right => &next_nodes.right,
        };
        num_moves += 1;
    }
    println!("Part 1 solution: {num_moves}");

    // Solution is built on the assumption that every node is ultimately looping through
    // the same route, and as long as we know the length of each of those routes, we can
    // calculate when the routes align.
    // Initialize 'current nodes' to starting nodes, aka nodes that end in 'A'
    let mut current_nodes = nodes_to_next_nodes
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .collect::<Vec<_>>();

    let mut node_route_first_z_lens: Vec<Option<usize>> = vec![None; current_nodes.len()];
    let mut num_moves: usize = 0;
    let instructions_len = instructions.len();
    while node_route_first_z_lens
        .iter()
        .any(|maybe_len| maybe_len.is_none())
    {
        let i = num_moves % instructions_len;
        let instruction = instructions[i];
        num_moves += 1;
        current_nodes = current_nodes
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let next_nodes = nodes_to_next_nodes.get(*node).unwrap();
                let new_node = match instruction {
                    Instruction::Left => &next_nodes.left,
                    Instruction::Right => &next_nodes.right,
                };
                if new_node.chars().last().unwrap() == 'Z' {
                    if node_route_first_z_lens[i].is_none() {
                        node_route_first_z_lens[i] = Some(num_moves);
                    }
                }
                new_node
            })
            .collect::<Vec<_>>();
    }

    let route_lens = node_route_first_z_lens
        .iter()
        .map(|n| n.unwrap())
        .collect::<Vec<_>>();

    // starting from max route len, brute-force the least common multiple (?)
    // note: this would be slightly faster if I removed the max number so I don't unnecessarily
    // check it every time
    let max_route_len = route_lens.iter().max().unwrap();
    let mut curr_route_len = 0;
    loop {
        curr_route_len += max_route_len;
        if route_lens.iter().all(|len| curr_route_len % len == 0) {
            break;
        }
    }
    println!("Part 2 solution: {curr_route_len}");
}
