use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn tilt_north_one_tile(grid: &Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, bool) {
    let grid_len = grid.len();
    let grid_width = grid[0].len();

    let mut new_grid = grid.clone();
    let mut changed = false;

    // starting iteration at y=1 not y=0
    for y in 1..grid_len {
        for x in 0..grid_width {
            let tile = new_grid[y][x];
            let tile_above = new_grid[y - 1][x];
            if tile == Tile::RoundRock && tile_above == Tile::Empty {
                new_grid[y - 1][x] = Tile::RoundRock;
                new_grid[y][x] = Tile::Empty;
                changed = true;
            }
        }
    }
    (new_grid, changed)
}

fn tilt_fully_north(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut changed = true;
    let mut grid = grid.clone();
    while changed {
        (grid, changed) = tilt_north_one_tile(&grid);
    }
    grid
}

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
    let file = File::open("resources/input_1").unwrap();
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

    let grid = tilt_fully_north(&grid);
    let score = score_grid(&grid);
    println!("Part 1 solution: {}", score);
}
