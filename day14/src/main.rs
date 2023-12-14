use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    CubeRock,
    RoundRock,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            'O' => Tile::RoundRock,
            '#' => Tile::CubeRock,
            '.' => Tile::Empty,
            _ => panic!("No tile type matches char '{}'", c),
        }
    }
}

fn speed_tilt_north(grid: &mut Vec<Vec<Tile>>) -> () {
    let grid_width = grid[0].len();
    let grid_len = grid.len();

    for x in 0..grid_width {
        let mut y_for_next_rock = 0;
        for y in 0..grid_len {
            match grid[y][x] {
                Tile::Empty => (),
                Tile::RoundRock => {
                    if y != y_for_next_rock {
                        grid[y_for_next_rock][x] = Tile::RoundRock;
                        grid[y][x] = Tile::Empty;
                    }
                    y_for_next_rock += 1;
                }
                Tile::CubeRock => {
                    y_for_next_rock = y + 1;
                }
            }
        }
    }
}

fn speed_tilt_south(grid: &mut Vec<Vec<Tile>>) -> () {
    let grid_width = grid[0].len();
    let grid_len = grid.len();

    for x in 0..grid_width {
        let mut y_for_next_rock = grid_len - 1;
        for y in (0..grid_len).rev() {
            match grid[y][x] {
                Tile::Empty => (),
                Tile::RoundRock => {
                    if y != y_for_next_rock {
                        grid[y_for_next_rock][x] = Tile::RoundRock;
                        grid[y][x] = Tile::Empty;
                    }
                    y_for_next_rock -= 1;
                }
                Tile::CubeRock => {
                    if y > 0 {
                        y_for_next_rock = y - 1;
                    }
                }
            }
        }
    }
}

fn speed_tilt_west(grid: &mut Vec<Vec<Tile>>) -> () {
    let grid_width = grid[0].len();
    let grid_len = grid.len();

    for y in 0..grid_len {
        let mut x_for_next_rock = 0;
        for x in 0..grid_width {
            match grid[y][x] {
                Tile::Empty => (),
                Tile::RoundRock => {
                    if x != x_for_next_rock {
                        grid[y][x_for_next_rock] = Tile::RoundRock;
                        grid[y][x] = Tile::Empty;
                    }
                    x_for_next_rock += 1;
                }
                Tile::CubeRock => {
                    x_for_next_rock = x + 1;
                }
            }
        }
    }
}

fn speed_tilt_east(grid: &mut Vec<Vec<Tile>>) -> () {
    let grid_width = grid[0].len();
    let grid_len = grid.len();

    for y in 0..grid_len {
        let mut x_for_next_rock = grid_width - 1;
        for x in (0..grid_width).rev() {
            match grid[y][x] {
                Tile::Empty => (),
                Tile::RoundRock => {
                    if x != x_for_next_rock {
                        grid[y][x_for_next_rock] = Tile::RoundRock;
                        grid[y][x] = Tile::Empty;
                    }
                    x_for_next_rock -= 1;
                }
                Tile::CubeRock => {
                    if x > 0 {
                        x_for_next_rock = x - 1;
                    }
                }
            }
        }
    }
}

fn rotate_grid(grid: &mut Vec<Vec<Tile>>, times: usize) {
    for _ in 0..times {
        speed_tilt_north(grid);
        speed_tilt_west(grid);
        speed_tilt_south(grid);
        speed_tilt_east(grid);
    }
}

// fn rotate_grid_memoized(grid: &Vec<Vec<Tile>>, times: usize) {
//     let grid_inputs_to_outputs: HashMap<&Vec<Vec<Tile>>, Vec<Vec<Tile>>> = HashMap::new();
//     let maybe_output = grid_inputs_to_outputs.get(grid);
//     match maybe_output {
//         Some(new_grid) => grid = new_grid,
//         None => {
//             let new_grid = grid.clone();
//             rotate_grid(&mut new_grid, 1);
//             grid_inputs_to_outputs.insert(grid, new_grid);
//         }
//     }
//     let grid_in = grid.clone();
// }

fn score_grid(grid: &Vec<Vec<Tile>>) -> usize {
    let grid_len = grid.len();
    grid.iter()
        .enumerate()
        .map(|(row_i, row)| {
            row.iter().filter(|t| **t == Tile::RoundRock).count() * (grid_len - row_i)
        })
        .sum::<usize>()
}

fn main() {
    let file = File::open("resources/sample_1").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        grid.push(
            line_content
                .chars()
                .map(Tile::from_char)
                .collect::<Vec<_>>(),
        );
    }

    let mut grid_copy = grid.clone();
    speed_tilt_north(&mut grid_copy);
    let part_1_score = score_grid(&grid_copy);
    println!("Part 1 solution: {}", part_1_score);

    rotate_grid(&mut grid, 1000000000);
    let part_2_score = score_grid(&grid);
    println!("Part 2 solution: {}", part_2_score);
    dbg!(&grid);
}
