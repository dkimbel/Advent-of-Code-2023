use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct ScratchCard {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    num_earned: i32,
}

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let mut cards: Vec<ScratchCard> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let halves = line_content.split(" | ").collect::<Vec<_>>();
        let first_nums_str = halves[0].split(": ").collect::<Vec<_>>()[1];
        let first_nums = first_nums_str
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let second_nums = halves[1]
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        cards.push(ScratchCard {
            winning_numbers: first_nums,
            numbers: second_nums,
            num_earned: 1,
        })
    }

    // don't panic on empty list
    let max_card_index = if cards.is_empty() { 0 } else { cards.len() - 1 };
    for i in 0..cards.len() {
        let card = cards[i].clone();
        let mut num_matches = 0;
        for num in card.numbers.iter() {
            if card.winning_numbers.contains(num) {
                num_matches += 1;
            }
        }

        let mut working_index = i + 1;
        while num_matches > 0 && working_index <= max_card_index {
            cards[working_index].num_earned += card.num_earned;
            working_index += 1;
            num_matches -= 1;
        }
    }
    let part_2_solution = cards.iter().map(|card| card.num_earned).sum::<i32>();
    println!("Part 2 solution: {part_2_solution}");
    // let mut scores: Vec<i32> = Vec::new();
    // for card in cards.iter() {
    //     let mut num_matches = 0;
    //     for num in card.numbers.iter() {
    //         if card.winning_numbers.contains(num) {
    //             num_matches += 1;
    //         }
    //     }
    //     let score: i32 = if num_matches == 0 {
    //         0
    //     } else {
    //         let base: i32 = 2;
    //         base.pow(num_matches - 1)
    //     };
    //     scores.push(score);
    // }
    // let part_1_solution = scores.iter().sum::<i32>();
    // println!("Part 1 solution: {part_1_solution}");
}
