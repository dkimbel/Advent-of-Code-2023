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

    println!("Part 1 solution: {num_moves}")
}
