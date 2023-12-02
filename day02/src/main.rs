use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let file = File::open("resources/sample_1").unwrap();
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);

    let mut games: Vec<Game> = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let line_halves = line_content.split(": ").collect::<Vec<_>>();
        let game_id = line_halves[0].split(" ").collect::<Vec<_>>()[1]
            .parse::<u32>()
            .unwrap();
        let mut rounds: Vec<GameRound> = Vec::new();
        let round_strs = line_halves[1].split("; ");
        for round_str in round_strs {
            let mut num_green = 0;
            let mut num_blue = 0;
            let mut num_red = 0;
            let nums_with_colors = round_str.split(", ");
            for num_with_color in nums_with_colors {
                let num_color_split = num_with_color.split(" ").collect::<Vec<_>>();
                let num_str = num_color_split[0];
                let color = num_color_split[1];
                let num = num_str.parse::<u32>().unwrap();
                match color {
                    "green" => num_green = num,
                    "red" => num_red = num,
                    "blue" => num_blue = num,
                    _ => panic!("Could not match color {}", color),
                };
            }
            let round = GameRound {
                num_green,
                num_red,
                num_blue,
            };
            rounds.push(round);
        }
        let game = Game {
            id: game_id,
            rounds,
        };
        games.push(game);
    }
    // let part_1_matching_games = games.iter().filter(|game| {
    //     !game
    //         .rounds
    //         .iter()
    //         .any(|round| round.num_green > 13 || round.num_blue > 14 || round.num_red > 12)
    // });
    // let part_1_solution = part_1_matching_games.map(|game| game.id).sum::<u32>();
    // println!("Part 1 solution: {}", part_1_solution);
    let mut max_round_values: Vec<GameRound> = Vec::new();
    for game in games {
        let mut max_green = 0;
        let mut max_blue = 0;
        let mut max_red = 0;
        for round in game.rounds {
            max_green = cmp::max(max_green, round.num_green);
            max_red = cmp::max(max_red, round.num_red);
            max_blue = cmp::max(max_blue, round.num_blue);
        }
        max_round_values.push(GameRound {
            num_red: max_red,
            num_blue: max_blue,
            num_green: max_green,
        });
    }
    let part_2_solution = max_round_values
        .iter()
        .map(|round| round.num_green * round.num_blue * round.num_red)
        .sum::<u32>();
    println!("Part 2 solution: {}", part_2_solution);
}

#[derive(Debug)]
struct GameRound {
    num_green: u32,
    num_red: u32,
    num_blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<GameRound>,
}
