use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct Race {
    time_ms: usize,
    record_distance_mm: usize,
}

fn main() {
    let filename = "resources/input_1";
    let part_1_solution = solve_part_1(filename);
    println!("Part 1 solution: {part_1_solution}");

    let part_2_solution = solve_part_2(filename);
    println!("Part 2 solution: {part_2_solution}");
}

fn solve_part_1(filename: &str) -> usize {
    let file_content = fs::read_to_string(filename).unwrap();
    let split = file_content.split("\n").collect::<Vec<_>>();

    let times = split[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = split[1]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut races: Vec<Race> = Vec::new();
    for (time, distance) in zip(times, distances) {
        races.push(Race {
            time_ms: time,
            record_distance_mm: distance,
        })
    }

    let mut nums_winning_options: Vec<usize> = Vec::new();
    for race in races.iter() {
        let mut possible_distances: Vec<usize> = Vec::new();
        for holdable_ms in 0..race.time_ms {
            let speed_mm_per_ms = holdable_ms;
            possible_distances.push(speed_mm_per_ms * (race.time_ms - holdable_ms));
        }
        nums_winning_options.push(
            possible_distances
                .iter()
                .filter(|d| **d > race.record_distance_mm)
                .count(),
        );
    }

    nums_winning_options.iter().product::<usize>()
}

fn solve_part_2(filename: &str) -> usize {
    let file_content = fs::read_to_string(filename).unwrap();
    let split = file_content.split("\n").collect::<Vec<_>>();

    let race_time_ms = split[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let record_distance_mm = split[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut num_winning_options = 0;
    for holdable_ms in 0..race_time_ms {
        let speed_mm_per_ms = holdable_ms;
        let distance = speed_mm_per_ms * (race_time_ms - holdable_ms);
        if distance > record_distance_mm {
            num_winning_options += 1;
        }
    }
    num_winning_options
}
