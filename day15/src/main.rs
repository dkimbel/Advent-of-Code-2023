use std::fs;

fn apply_hash_algorithm(input: &str) -> u8 {
    let mut result: u64 = 0;
    for c in input.chars() {
        let ascii_code = c as u8;
        result += ascii_code as u64;
        result *= 17;
        result %= 256;
    }
    result as u8
}

fn main() {
    let mut file_content = fs::read_to_string("resources/input_1").unwrap();
    // strip trailing newline
    file_content.truncate(file_content.len() - 1);
    let strs = file_content.split(",");
    let total = strs
        .map(apply_hash_algorithm)
        .map(|n| n as u64)
        .sum::<u64>();
    println!("Part 1 solution: {total}");
}
