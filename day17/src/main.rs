use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct SearchState {
    coords: Coords,
    direction: Direction,
    consecutive_moves_same_direction: u32,
    total_cost_incurred: usize,
    visited_tiles: HashSet<Coords>,
}

// Reusing from Day 17
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

fn get_adjacent_directions(direction: Direction) -> (Direction, Direction) {
    use Direction::*;
    match direction {
        Up => (Left, Right),
        Down => (Left, Right),
        Left => (Up, Down),
        Right => (Up, Down),
    }
}

fn adjust_cost_for_distance_from_goal(
    curr_cost: usize,
    curr_coords: Coords,
    end_coords: Coords,
) -> usize {
    let distance = (end_coords.y - curr_coords.y) + (end_coords.x - curr_coords.x);
    // only charge 1 for the nearest-to-goal couple tiles to be safe, then 2 for the next two
    // nearest, and 3 otherwise; should be okay since low-cost tiles are quite rare
    let cost_diff = if distance <= 2 {
        distance
    } else if distance <= 4 {
        (distance - 2) * 2 + 2
    } else {
        (distance - 4) * 3 + 6
    };
    curr_cost + cost_diff
}

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in reader.lines() {
        let row = line
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        grid.push(row);
    }

    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    println!("{} {}", max_x, max_y);
    let end_coords = Coords { x: max_x, y: max_y };
    let mut lowest_full_cost: usize = usize::MAX;
    let mut lowest_cost_per_coord_state: HashMap<(Coords, Direction, u32), usize> = HashMap::new();
    let mut search_stack: Vec<SearchState> = vec![
        SearchState {
            coords: Coords { x: 0, y: 0 },
            consecutive_moves_same_direction: 0,
            direction: Direction::Down,
            total_cost_incurred: 0,
            visited_tiles: HashSet::new(),
        },
        SearchState {
            coords: Coords { x: 0, y: 0 },
            consecutive_moves_same_direction: 0,
            direction: Direction::Right,
            total_cost_incurred: 0,
            visited_tiles: HashSet::new(),
        },
    ];

    while let Some(mut search_state) = search_stack.pop() {
        // start by executing whatever move was inevitable based on current direction
        let maybe_new_coords =
            maybe_next_coords(search_state.coords, search_state.direction, max_x, max_y);
        if maybe_new_coords.is_some() {
            search_state.coords = maybe_new_coords.unwrap();
            if !search_state.visited_tiles.contains(&search_state.coords) {
                search_state.visited_tiles.insert(search_state.coords);
                search_state.consecutive_moves_same_direction += 1;
                search_state.total_cost_incurred +=
                    grid[search_state.coords.y][search_state.coords.x];
                let adjusted_current_cost = adjust_cost_for_distance_from_goal(
                    search_state.total_cost_incurred,
                    search_state.coords,
                    end_coords,
                );
                if search_state.coords == end_coords {
                    lowest_full_cost =
                        std::cmp::min(lowest_full_cost, search_state.total_cost_incurred);
                    println!("Lowest full cost: {}", lowest_full_cost);
                } else if adjusted_current_cost < lowest_full_cost {
                    // stop if there was already a lower-cost version of comparable state
                    for consec in 0..search_state.consecutive_moves_same_direction + 1 {
                        let coord_state = (search_state.coords, search_state.direction, consec);
                        if let Some(cost) = lowest_cost_per_coord_state.get(&coord_state) {
                            if *cost < adjusted_current_cost {
                                continue;
                            }
                        }
                    }
                    // update lowest-cost state of current and any *worse* cases
                    for consec in search_state.consecutive_moves_same_direction..4 {
                        let coord_state = (search_state.coords, search_state.direction, consec);
                        let maybe_curr_lowest = lowest_cost_per_coord_state.get(&coord_state);
                        if maybe_curr_lowest.is_none()
                            || *maybe_curr_lowest.unwrap() > search_state.total_cost_incurred
                        {
                            lowest_cost_per_coord_state
                                .insert(coord_state, search_state.total_cost_incurred);
                        }
                    }
                    let adjacent_directions = get_adjacent_directions(search_state.direction);
                    let mut first_new_search = search_state.clone();
                    first_new_search.direction = adjacent_directions.0;
                    first_new_search.consecutive_moves_same_direction = 0;
                    search_stack.push(first_new_search);

                    let mut second_new_search = search_state.clone();
                    second_new_search.direction = adjacent_directions.1;
                    second_new_search.consecutive_moves_same_direction = 0;
                    search_stack.push(second_new_search);

                    if search_state.consecutive_moves_same_direction < 3 {
                        search_stack.push(search_state);
                    }
                }
            }
        }
    }

    println!("Part 1 solution: {lowest_full_cost}");
}
