use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct BeamState {
    coords: Coords,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    Mirror45,
    Mirror135,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        use Tile::*;
        match c {
            '.' => Empty,
            '|' => VerticalSplitter,
            '-' => HorizontalSplitter,
            '/' => Mirror45,
            '\\' => Mirror135,
            _ => panic!("No tile type matches char {c}"),
        }
    }
}

fn get_new_beams(beam: BeamState, grid: &Vec<Vec<Tile>>) -> (Option<BeamState>, Option<BeamState>) {
    let tile = &grid[beam.coords.y][beam.coords.x];
    let coords = beam.coords;
    use Direction::*;
    use Tile::*;
    let new_directions: (Direction, Option<Direction>) = match (tile, beam.direction) {
        (Empty, Right) => (Right, None),
        (Empty, Left) => (Left, None),
        (Empty, Down) => (Down, None),
        (Empty, Up) => (Up, None),
        (VerticalSplitter, Right) => (Up, Some(Down)),
        (VerticalSplitter, Left) => (Up, Some(Down)),
        (VerticalSplitter, Up) => (Up, None),
        (VerticalSplitter, Down) => (Down, None),
        (HorizontalSplitter, Right) => (Right, None),
        (HorizontalSplitter, Left) => (Left, None),
        (HorizontalSplitter, Up) => (Right, Some(Left)),
        (HorizontalSplitter, Down) => (Right, Some(Left)),
        (Mirror45, Right) => (Up, None),
        (Mirror45, Left) => (Down, None),
        (Mirror45, Up) => (Right, None),
        (Mirror45, Down) => (Left, None),
        (Mirror135, Right) => (Down, None),
        (Mirror135, Left) => (Up, None),
        (Mirror135, Up) => (Left, None),
        (Mirror135, Down) => (Right, None),
    };
    let max_x = &grid[0].len() - 1;
    let max_y = &grid.len() - 1;
    let maybe_first_beam =
        maybe_next_coords(coords, new_directions.0, max_x, max_y).map(|coords| BeamState {
            coords,
            direction: new_directions.0,
        });
    let maybe_second_beam = new_directions
        .1
        .map(|dir| {
            maybe_next_coords(coords, dir, max_x, max_y).map(|coords| BeamState {
                coords,
                direction: dir,
            })
        })
        .flatten();
    (maybe_first_beam, maybe_second_beam)
}

fn maybe_next_coords(
    coords: Coords,
    direction: Direction,
    max_x: usize,
    max_y: usize,
) -> Option<Coords> {
    use Direction::*;
    let x = coords.x;
    let y = coords.y;
    match direction {
        Up => {
            if coords.y > 0 {
                Some(Coords { x, y: y - 1 })
            } else {
                None
            }
        }
        Down => {
            if coords.y < max_y {
                Some(Coords { x, y: y + 1 })
            } else {
                None
            }
        }
        Right => {
            if coords.x < max_x {
                Some(Coords { x: x + 1, y })
            } else {
                None
            }
        }
        Left => {
            if coords.x > 0 {
                Some(Coords { x: x - 1, y })
            } else {
                None
            }
        }
    }
}

fn get_num_energized_from_starting_beam(beam: BeamState, grid: &Vec<Vec<Tile>>) -> usize {
    let mut already_processed_beam_states: HashSet<BeamState> = HashSet::new();
    let mut beams = vec![beam];

    while let Some(beam) = beams.pop() {
        if already_processed_beam_states.contains(&beam) {
            continue;
        }
        already_processed_beam_states.insert(beam);
        let new_beams: (Option<BeamState>, Option<BeamState>) = get_new_beams(beam, &grid);
        if new_beams.0.is_some() {
            beams.push(new_beams.0.unwrap());
        }
        if new_beams.1.is_some() {
            beams.push(new_beams.1.unwrap());
        }
    }

    let visited_coords: HashSet<Coords> =
        HashSet::from_iter(already_processed_beam_states.iter().map(|beam| beam.coords));
    visited_coords.len()
}

fn all_possible_starting_beams(grid: &Vec<Vec<Tile>>) -> Vec<BeamState> {
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let mut all_possible_starting_beams: Vec<BeamState> = Vec::new();
    for possible_y in 0..(max_y + 1) {
        all_possible_starting_beams.push(BeamState {
            coords: Coords {
                x: 0,
                y: possible_y,
            },
            direction: Direction::Right,
        });
        all_possible_starting_beams.push(BeamState {
            coords: Coords {
                x: max_x,
                y: possible_y,
            },
            direction: Direction::Left,
        });
    }
    for possible_x in 0..(max_x + 1) {
        all_possible_starting_beams.push(BeamState {
            coords: Coords {
                x: possible_x,
                y: 0,
            },
            direction: Direction::Down,
        });
        all_possible_starting_beams.push(BeamState {
            coords: Coords {
                x: possible_x,
                y: max_y,
            },
            direction: Direction::Up,
        });
    }
    all_possible_starting_beams
}

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<Tile>> = Vec::new();

    for line in reader.lines() {
        let row: Vec<Tile> = line.unwrap().chars().map(Tile::from_char).collect();
        grid.push(row);
    }

    let part_1_beam = BeamState {
        coords: Coords { x: 0, y: 0 },
        direction: Direction::Right,
    };
    let part_1_num_energized = get_num_energized_from_starting_beam(part_1_beam, &grid);
    println!("Part 1 solution: {part_1_num_energized}");

    let all_possible_starting_beams = all_possible_starting_beams(&grid);
    let max_num_energized = all_possible_starting_beams
        .iter()
        .map(|beam| get_num_energized_from_starting_beam(*beam, &grid))
        .max()
        .unwrap();
    println!("Part 2 solution: {max_num_energized}");
}
