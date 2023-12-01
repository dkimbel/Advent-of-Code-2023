use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn num_str_to_digit_str(num_str: &str) -> &str {
    match num_str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => num_str,
    }
}

fn main() {
    let file = File::open("resources/input_1").unwrap();
    // let file = File::open("resources/sample_1").unwrap();
    // let file = File::open("resources/sample_2").unwrap();
    // let file = File::open("resources/sample_3").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    let reversed_re = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d").unwrap();
    let mut combined_nums = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let matches: Vec<&str> = re.find_iter(line_content).map(|m| m.as_str()).collect();
        let first_num_str = matches[0];
        let first_num_parsed = num_str_to_digit_str(first_num_str);
        let reversed_line_content = line_content.chars().rev().collect::<String>();
        let reversed_matches: Vec<&str> = reversed_re
            .find_iter(&reversed_line_content)
            .map(|m| m.as_str())
            .collect();
        let last_num_str = reversed_matches[0].chars().rev().collect::<String>();
        let last_num_parsed = num_str_to_digit_str(&last_num_str);
        combined_nums.push(
            format!("{}{}", first_num_parsed, last_num_parsed)
                .parse::<i64>()
                .unwrap(),
        );
    }
    let sum: i64 = combined_nums.iter().sum();
    println!("{sum}");

    // let mut combined_nums = Vec::new();
    // for line in reader.lines() {
    //     let line_content = &line.unwrap();
    //     let mut first_num = '0';
    //     let mut last_num = '0';
    //     let mut has_seen_num = false;
    //     for char in line_content.chars() {
    //         match char.to_digit(10) {
    //             Some(d) => {
    //                 if !has_seen_num {
    //                     first_num = char;
    //                     has_seen_num = true;
    //                 }
    //                 last_num = char;
    //             }
    //             _ => {}
    //         }
    //     }
    //     combined_nums.push(format!("{}{}", first_num, last_num).parse::<u32>().unwrap());
    //     first_num = '0';
    //     last_num = '0';
    //     has_seen_num = false;
    // }
    // let sum: u32 = combined_nums.iter().sum();
    // println!("{}", sum);
}
