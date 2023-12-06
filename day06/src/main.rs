use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct Race {
    time_ms: usize,
    record_distance_mm: usize,
}

fn main() {
    let file_content = fs::read_to_string("resources/input_1").unwrap();
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

    let part_1_solution = nums_winning_options.iter().product::<usize>();
    println!("Part 1 solution: {part_1_solution}");
}
