use std::fs::File;
use std::io::{BufRead, BufReader};

fn generate_diffs(readings: &[i64]) -> Vec<i64> {
    readings
        .windows(2)
        .map(|slice| {
            if slice.len() == 2 {
                slice[1] - slice[0]
            } else if slice.len() == 1 {
                slice[0]
            } else {
                panic!("Ran out of items in slice!");
            }
        })
        .collect::<Vec<_>>()
}

fn infer_last_reading(readings: &[i64]) -> i64 {
    // base case
    if readings.iter().all(|r| *r == 0) {
        return 0;
    } else {
        let diffs = generate_diffs(readings);
        return readings.iter().last().unwrap() + infer_last_reading(&diffs);
    }
}

fn infer_first_reading(readings: &[i64]) -> i64 {
    // base case
    if readings.iter().all(|r| *r == 0) {
        return 0;
    } else {
        let diffs = generate_diffs(readings);
        return readings[0] - infer_first_reading(&diffs);
    }
}

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);
    let mut histories: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let history = line_content
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        histories.push(history);
    }

    let extrapolations = histories
        .iter()
        .map(|h| infer_last_reading(h))
        .collect::<Vec<_>>();
    let part_1_solution = extrapolations.iter().sum::<i64>();
    println!("Part 1 solution: {part_1_solution}");

    let first_reading_extrapolations = histories
        .iter()
        .map(|h| infer_first_reading(h))
        .collect::<Vec<_>>();
    let part_2_solution = first_reading_extrapolations.iter().sum::<i64>();
    println!("Part 2 solution: {part_2_solution}");
}
