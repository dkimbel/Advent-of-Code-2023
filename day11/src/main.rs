use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
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

    let galaxy_ids_to_expanded_coords: HashMap<usize, Coords> = galaxy_ids_to_unexpanded_coords
        .into_iter()
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
                id,
                Coords {
                    x: c.x + extra_columns_to_left,
                    y: c.y + extra_rows_below,
                },
            )
        })
        .collect();

    let max_galaxy_id = galaxy_ids_to_expanded_coords.len();
    let mut galaxy_id_pairs: Vec<(usize, usize)> = Vec::new();
    for first_id in 1..max_galaxy_id {
        for second_id in (first_id + 1)..(max_galaxy_id + 1) {
            galaxy_id_pairs.push((first_id, second_id));
        }
    }

    let galaxy_id_pairs_to_min_distances: HashMap<(usize, usize), usize> = galaxy_id_pairs
        .into_iter()
        .map(|id_pair| {
            let first_coords = galaxy_ids_to_expanded_coords.get(&id_pair.0).unwrap();
            let second_coords = galaxy_ids_to_expanded_coords.get(&id_pair.1).unwrap();
            let greater_x = std::cmp::max(first_coords.x, second_coords.x);
            let lesser_x = std::cmp::min(first_coords.x, second_coords.x);
            let greater_y = std::cmp::max(first_coords.y, second_coords.y);
            let lesser_y = std::cmp::min(first_coords.y, second_coords.y);
            let distance = (greater_x - lesser_x) + (greater_y - lesser_y);
            (id_pair, distance)
        })
        .collect();

    let combined_distances = galaxy_id_pairs_to_min_distances.values().sum::<usize>();
    println!("Part 1 solution: {combined_distances}");
}
