use std::collections::{HashMap, HashSet};
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

fn all_nodes_are_end_nodes(nodes: &Vec<&String>, num_moves: usize) -> bool {
    nodes.iter().all(|node| node.chars().last().unwrap() == 'Z')
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

    // Solution is built on the assumption that every node is ultimately looping through
    // the same route, and as long as we know the length of each of those routes, we can
    // calculate when the routes align.
    // From observing program outputs, the length of this recurring loop length should be
    // calculated second_z_lenth - first_z_length, where e.g. 'first z length' is the number
    // of moves when the route first reaches a Z-ending node.
    // Initialize 'current nodes' to starting nodes, aka nodes that end in 'A'
    let mut current_nodes = nodes_to_next_nodes
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .collect::<Vec<_>>();

    let mut node_route_first_z_lens: Vec<Option<usize>> = vec![None; current_nodes.len()];
    // let mut node_route_second_z_lens: Vec<Option<usize>> = vec![None; current_nodes.len()];

    println!("Initial current nodes: {:?}", current_nodes);

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
                    // } else if node_route_second_z_lens[i].is_none() {
                    //     node_route_second_z_lens[i] = Some(num_moves);
                    // }
                }
                new_node
            })
            .collect::<Vec<_>>();

        if num_moves < 10 || num_moves % 10000 == 0 {
            println!("Move {} current nodes: {:?}", num_moves, current_nodes);
        }
    }

    println!("First lens: {:?}", node_route_first_z_lens);
    // println!("Second lens: {:?}", node_route_second_z_lens);
    let route_lens = node_route_first_z_lens
        .iter()
        .map(|n| n.unwrap())
        .collect::<Vec<_>>();
    // let node_route_first_z_lens_unwrapped = node_route_first_z_lens
    // .iter()
    // .map(|n| n.unwrap())
    // .collect::<Vec<_>>();
    // let node_route_second_z_lens_unwrapped = node_route_second_z_lens
    // .iter()
    // .map(|n| n.unwrap())
    // .collect::<Vec<_>>();
    // let route_lens = std::iter::zip(
    // node_route_first_z_lens_unwrapped.clone(),
    // node_route_second_z_lens_unwrapped,
    // )
    // .map(|(first, second)| second - first)
    // .collect::<Vec<usize>>();
    println!("Node route lens: {:?}", route_lens);

    // start from max route len, try to brute-force a number that's divisible by all of them
    // note: this would be slightly faster if I removed the max number so I don't unnecessarily
    // check it every time
    let max_route_len = route_lens.iter().max().unwrap();
    let mut curr_route_len = 0;
    let mut i = 0;
    loop {
        curr_route_len += max_route_len;
        i += 1;
        if route_lens.iter().all(|len| curr_route_len % len == 0) {
            break;
        }
        if i % 10000 == 0 {
            println!("Curr route len: {}", curr_route_len)
        }
    }
    println!("Part 2 solution: {curr_route_len}");

    // brute force-ish
    // arrived at 3325072085056, but apparently this is wrong (also tried 3325072085057)
    // let mut turn_nums_on_z = node_route_first_z_lens_unwrapped;
    // let mut i = 0;
    // loop {
    //     let first_node_turn_num = turn_nums_on_z[0];
    //     if turn_nums_on_z.iter().all(|pos| *pos == first_node_turn_num) {
    //         break;
    //     }
    //     if *turn_nums_on_z.iter().min().unwrap() == first_node_turn_num {
    //         turn_nums_on_z[0] += route_lens[0];
    //     } else {
    //         for i in 1..turn_nums_on_z.len() {
    //             if turn_nums_on_z[i] < first_node_turn_num {
    //                 turn_nums_on_z[i] += route_lens[i];
    //             }
    //         }
    //     }
    //     i += 1;
    //     if i % 10000 == 0 {
    //         println!("{:?}", turn_nums_on_z);
    //     }
    // }
    // println!("Part 2 solution: {}", turn_nums_on_z[0]);

    // // find lowest common multiple of all route lengths
    // // to achieve this, according to the internet:
    // //   - first find greatest common divisor
    // //   - then multiply all the numbers together and divide by greatest common divisor
    // // let route_lens = node_route_lengths
    // let route_lens = node_route_first_z_lens
    //     .iter()
    //     .map(|n| n.unwrap() + 1) // check for off by one
    //     .collect::<Vec<_>>();

    // let unique_route_lens_set: HashSet<usize> = HashSet::from_iter(route_lens.into_iter());
    // let mut unique_route_lens: Vec<usize> = unique_route_lens_set.into_iter().collect();
    // let greatest_route_len = *unique_route_lens.iter().max().unwrap();
    // let lowest_route_len = *unique_route_lens.iter().min().unwrap();
    // // brute-force the greatest common divisor
    // let mut gcd = 1;
    // let mut curr_num = 1;
    // while curr_num < lowest_route_len {
    //     curr_num += 1;
    //     if unique_route_lens.iter().all(|len| len % curr_num == 0) {
    //         gcd = curr_num;
    //     }
    // }

    // println!("Route lens: {:?}", unique_route_lens);
    // println!("GCD: {}", gcd);
    // let max_solution = gcd * greatest_route_len;
    // let mut n = greatest_route_len;
    // loop {
    //     if unique_route_lens.iter().all(|len| len % n == 0) {
    //         println!("Real part 2 solution: {}", n);
    //         return;
    //     }
    //     n += gcd;
    // }
    // let solution = gcd * greatest_route_len;
    // println!("Part 2 solution: {}", solution);

    // Part 1 code below
    //
    // let mut num_moves: usize = 0;
    // let instructions_len = instructions.len();
    // let mut curr_node = "AAA";
    // while curr_node != "ZZZ" {
    //     let i = num_moves % instructions_len;
    //     let next_nodes = nodes_to_next_nodes.get(curr_node).unwrap();
    //     let instruction = instructions[i];
    //     curr_node = match instruction {
    //         Instruction::Left => &next_nodes.left,
    //         Instruction::Right => &next_nodes.right,
    //     };
    //     num_moves += 1;
    // }

    // println!("Part 1 solution: {num_moves}")
}
