use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

fn expand_universe_by_factor(
    galaxy_ids_to_coords: &HashMap<usize, Coords>,
    indexes_to_insert_column_after: &[usize],
    indexes_to_insert_row_after: &[usize],
    expansion_factor: usize,
) -> HashMap<usize, Coords> {
    galaxy_ids_to_coords
        .iter()
        .map(|(id, c)| {
            let extra_columns_to_left = indexes_to_insert_column_after
                .iter()
                .filter(|x| **x < c.x)
                .count();
            let extra_rows_below = indexes_to_insert_row_after
                .iter()
                .filter(|y| **y < c.y)
                .count();
            (
                *id,
                Coords {
                    x: c.x + (extra_columns_to_left * (expansion_factor - 1)),
                    y: c.y + (extra_rows_below * (expansion_factor - 1)),
                },
            )
        })
        .collect()
}

fn calculate_total_distance(galaxy_ids_to_coords: &HashMap<usize, Coords>) -> usize {
    let max_galaxy_id = galaxy_ids_to_coords.len();
    let mut galaxy_id_pairs: Vec<(usize, usize)> = Vec::new();
    for first_id in 1..max_galaxy_id {
        for second_id in (first_id + 1)..(max_galaxy_id + 1) {
            galaxy_id_pairs.push((first_id, second_id));
        }
    }

    let galaxy_id_pairs_to_min_distances: HashMap<(usize, usize), usize> = galaxy_id_pairs
        .into_iter()
        .map(|id_pair| {
            let first_coords = galaxy_ids_to_coords.get(&id_pair.0).unwrap();
            let second_coords = galaxy_ids_to_coords.get(&id_pair.1).unwrap();
            let greater_x = std::cmp::max(first_coords.x, second_coords.x);
            let lesser_x = std::cmp::min(first_coords.x, second_coords.x);
            let greater_y = std::cmp::max(first_coords.y, second_coords.y);
            let lesser_y = std::cmp::min(first_coords.y, second_coords.y);
            let distance = (greater_x - lesser_x) + (greater_y - lesser_y);
            (id_pair, distance)
        })
        .collect();

    galaxy_id_pairs_to_min_distances.values().sum::<usize>()
}

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);
    let mut galaxy_id = 1;
    let mut galaxy_ids_to_unexpanded_coords: HashMap<usize, Coords> = HashMap::new();
    let mut max_galaxy_unexpanded_x_coord = 0;
    let mut max_galaxy_unexpanded_y_coord = 0;
    for (y, line) in reader.lines().enumerate() {
        let line_content = &line.unwrap();
        for (x, char) in line_content.chars().enumerate() {
            if char == '#' {
                galaxy_ids_to_unexpanded_coords.insert(galaxy_id, Coords { x, y });
                galaxy_id += 1;
                max_galaxy_unexpanded_y_coord = std::cmp::max(y, max_galaxy_unexpanded_y_coord);
                max_galaxy_unexpanded_x_coord = std::cmp::max(x, max_galaxy_unexpanded_x_coord);
            }
        }
    }

    let all_galaxy_unexpanded_y_coords = galaxy_ids_to_unexpanded_coords
        .values()
        .map(|c| c.y)
        .collect::<HashSet<_>>();
    let mut indexes_to_insert_row_after: Vec<usize> = Vec::new();
    for y in 0..(max_galaxy_unexpanded_y_coord + 1) {
        if !all_galaxy_unexpanded_y_coords.contains(&y) {
            indexes_to_insert_row_after.push(y);
        }
    }

    let all_galaxy_unexpanded_x_coords = galaxy_ids_to_unexpanded_coords
        .values()
        .map(|c| c.x)
        .collect::<HashSet<_>>();
    let mut indexes_to_insert_column_after: Vec<usize> = Vec::new();
    for x in 0..(max_galaxy_unexpanded_x_coord + 1) {
        if !all_galaxy_unexpanded_x_coords.contains(&x) {
            indexes_to_insert_column_after.push(x);
        }
    }

    let part_1_galaxy_ids_to_expanded_coords = expand_universe_by_factor(
        &galaxy_ids_to_unexpanded_coords,
        &indexes_to_insert_column_after,
        &indexes_to_insert_row_after,
        2,
    );
    let part_1_solution = calculate_total_distance(&part_1_galaxy_ids_to_expanded_coords);
    println!("Part 1 solution: {part_1_solution}");

    let part_2_galaxy_ids_to_expanded_coords = expand_universe_by_factor(
        &galaxy_ids_to_unexpanded_coords,
        &indexes_to_insert_column_after,
        &indexes_to_insert_row_after,
        1000000,
    );
    let part_2_solution = calculate_total_distance(&part_2_galaxy_ids_to_expanded_coords);
    println!("Part 2 solution: {part_2_solution}");
}
