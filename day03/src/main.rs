use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

struct PartNumber {
    value: u32,
    surrounding_coords_incl_diagonal: Vec<Coords>,
}

fn main() {
    let radix = 10;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_content = &line.unwrap();
        grid.push(line_content.chars().collect::<Vec<_>>());
    }

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut gear_coords: Vec<Coords> = Vec::new();
    let mut curr_num_chars: Vec<char> = Vec::new();
    let mut indices_to_check: Vec<Coords> = Vec::new();
    // we assume that every row has the same length
    let num_cols = grid[0].len();
    let num_rows = grid.len();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            let curr_char_is_digit = char.is_digit(radix);
            if curr_char_is_digit {
                curr_num_chars.push(*char);
            } else if *char == '*' {
                gear_coords.push(Coords {
                    x: col_index,
                    y: row_index,
                });
            }

            // if we've just reached the end of a row or we've just reached the end of a
            // sequence of digits, we need to process our current number
            let num_len = curr_num_chars.len();
            let line_end = col_index + 1 == num_cols;
            if (line_end || !char.is_digit(radix)) && num_len > 0 {
                // check surroundings and add part number if appropriate
                let curr_num = curr_num_chars
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                // determine what all the surrounding coords are that we need to check (including diagonals)
                // first: deal with row above the current row
                let end_of_num_col_index = if line_end && curr_char_is_digit {
                    col_index
                } else {
                    col_index - 1
                };
                if row_index > 0 {
                    // work left to right in row
                    if end_of_num_col_index > num_len {
                        indices_to_check.push(Coords {
                            x: end_of_num_col_index - num_len,
                            y: row_index - 1,
                        });
                    }
                    for i in 0..num_len {
                        indices_to_check.push(Coords {
                            x: end_of_num_col_index - ((num_len - 1) - i),
                            y: row_index - 1,
                        })
                    }
                    if end_of_num_col_index + 1 < num_cols {
                        indices_to_check.push(Coords {
                            x: end_of_num_col_index + 1,
                            y: row_index - 1,
                        })
                    }
                }
                // second: deal with current row
                if end_of_num_col_index > num_len {
                    indices_to_check.push(Coords {
                        x: end_of_num_col_index - num_len,
                        y: row_index,
                    });
                }
                for i in 0..num_len {
                    indices_to_check.push(Coords {
                        x: end_of_num_col_index - ((num_len - 1) - i),
                        y: row_index,
                    })
                }
                if end_of_num_col_index + 1 < num_cols {
                    indices_to_check.push(Coords {
                        x: end_of_num_col_index + 1,
                        y: row_index,
                    });
                }
                // third and last: deal with row below
                if row_index + 1 < num_rows {
                    if end_of_num_col_index > num_len {
                        indices_to_check.push(Coords {
                            x: end_of_num_col_index - num_len,
                            y: row_index + 1,
                        });
                    }
                    for i in 0..num_len {
                        indices_to_check.push(Coords {
                            x: end_of_num_col_index - ((num_len - 1) - i),
                            y: row_index + 1,
                        })
                    }
                    if end_of_num_col_index + 1 < num_cols {
                        indices_to_check.push(Coords {
                            x: end_of_num_col_index + 1,
                            y: row_index + 1,
                        });
                    }
                }

                for coords in indices_to_check.iter() {
                    let grid_item = grid[coords.y][coords.x];
                    if !grid_item.is_digit(radix) && grid_item != '.' {
                        part_numbers.push(PartNumber {
                            value: curr_num,
                            surrounding_coords_incl_diagonal: indices_to_check.clone(),
                        });
                        break;
                    }
                }

                indices_to_check.clear();
                curr_num_chars.clear();
            }
        }
    }
    // let part_1_solution = part_numbers
    //     .iter()
    //     .map(|part_num| part_num.value)
    //     .sum::<u32>();
    // println!("Part 1 solution: {part_1_solution}");

    let mut gear_ratios: Vec<u32> = Vec::new();
    let mut part_numbers_adjacent_to_current_gear: Vec<u32> = Vec::new();
    // TODO: find gears that have exactly two adjacent nums
    // TODO: calculate and sum gear scores
    for gear in gear_coords {
        for part_number in part_numbers.iter() {
            if part_number.surrounding_coords_incl_diagonal.contains(&gear) {
                part_numbers_adjacent_to_current_gear.push(part_number.value);
            }
        }
        if part_numbers_adjacent_to_current_gear.len() == 2 {
            let gear_ratio =
                part_numbers_adjacent_to_current_gear[0] * part_numbers_adjacent_to_current_gear[1];
            gear_ratios.push(gear_ratio);
        }
        part_numbers_adjacent_to_current_gear.clear();
    }
    let part_2_solution = gear_ratios.into_iter().sum::<u32>();
    println!("Part 2 solution: {part_2_solution}");
}
