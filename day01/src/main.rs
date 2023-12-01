use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);

    let mut combined_nums = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let mut first_num = '0';
        let mut last_num = '0';
        let mut has_seen_num = false;
        for char in line_content.chars() {
            match char.to_digit(10) {
                Some(d) => {
                    if !has_seen_num {
                        first_num = char;
                        has_seen_num = true;
                    }
                    last_num = char;
                }
                _ => {}
            }
        }
        combined_nums.push(format!("{}{}", first_num, last_num).parse::<u32>().unwrap());
        first_num = '0';
        last_num = '0';
        has_seen_num = false;
    }
    let sum: u32 = combined_nums.iter().sum();
    println!("{}", sum);
}
