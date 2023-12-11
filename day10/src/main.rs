use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TileAccessibility {
    UnenclosedGround,
    MainLoopEnclosedGround,
    MainLoopPipe,
}

#[derive(Debug)]
struct SearchParams {
    route: Vec<Coords>,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
// represents either a single set of ground coords, or two sets of pipe coords plus the
// direction we entered from
struct CoordsMaybePipe {
    coords: Coords,
    additional_pipe_coords: Option<Coords>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Ground,
    StartPipe,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthEastPipe,
    SouthWestPipe,
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        use Tile::*;
        match c {
            '.' => Some(Ground),
            'S' => Some(StartPipe),
            '|' => Some(VerticalPipe),
            '-' => Some(HorizontalPipe),
            'L' => Some(NorthEastPipe),
            'J' => Some(NorthWestPipe),
            '7' => Some(SouthWestPipe),
            'F' => Some(SouthEastPipe),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        use Tile::*;
        match self {
            Ground => '.',
            StartPipe => 'S',
            VerticalPipe => '|',
            HorizontalPipe => '-',
            NorthEastPipe => 'L',
            NorthWestPipe => 'J',
            SouthWestPipe => '7',
            SouthEastPipe => 'F',
        }
    }

    fn connects_south(&self) -> bool {
        use Tile::*;
        match self {
            StartPipe => true,
            VerticalPipe => true,
            SouthEastPipe => true,
            SouthWestPipe => true,
            _ => false,
        }
    }

    fn connects_north(&self) -> bool {
        use Tile::*;
        match self {
            StartPipe => true,
            VerticalPipe => true,
            NorthEastPipe => true,
            NorthWestPipe => true,
            _ => false,
        }
    }

    fn connects_east(&self) -> bool {
        use Tile::*;
        match self {
            StartPipe => true,
            HorizontalPipe => true,
            NorthEastPipe => true,
            SouthEastPipe => true,
            _ => false,
        }
    }

    fn connects_west(&self) -> bool {
        use Tile::*;
        match self {
            StartPipe => true,
            HorizontalPipe => true,
            NorthWestPipe => true,
            SouthWestPipe => true,
            _ => false,
        }
    }
}

fn display_grid_with_known_accessibilities(
    grid: &Vec<Vec<Tile>>,
    accessibilities: &Vec<Vec<Option<TileAccessibility>>>,
) -> String {
    let mut printable: String = String::new();
    for (y, row) in grid.iter().enumerate() {
        let row_str = row
            .iter()
            .enumerate()
            .map(|(x, tile)| {
                if *tile == Tile::Ground {
                    let maybe_accessibility = accessibilities[y][x];
                    match maybe_accessibility {
                        Some(TileAccessibility::UnenclosedGround) => 'O',
                        Some(TileAccessibility::MainLoopEnclosedGround) => 'I',
                        _ => tile.to_char(),
                    }
                } else {
                    tile.to_char()
                }
            })
            .collect::<String>();
        printable = format!("{}{}\n", printable, row_str);
    }
    format!("{}\n", printable)
}

fn allows_passage_between(
    maybe_first: Option<(Coords, Tile)>,
    maybe_second: Option<(Coords, Tile)>,
) -> bool {
    if maybe_first.is_none() || maybe_second.is_none() {
        return false;
    }
    let first = maybe_first.unwrap();
    let second = maybe_second.unwrap();
    pipe_pair_allows_passage_north_or_south(first, second)
        || pipe_pair_allows_passage_east_or_west(first, second)
}

fn direction_of_second_from_first(first: Coords, second: Coords) -> Direction {
    // panics if coords aren't adjacent
    if first.y == second.y {
        // east or west (or invalid)
        let greater_x = std::cmp::max(first.x, second.x);
        let lesser_x = std::cmp::min(first.x, second.x);
        if greater_x - lesser_x == 1 {
            if greater_x == second.x {
                return Direction::East;
            } else {
                return Direction::West;
            }
        }
    } else if first.x == second.x {
        // north or south (or invalid)
        let greater_y = std::cmp::max(first.y, second.y);
        let lesser_y = std::cmp::min(first.y, second.y);
        if greater_y - lesser_y == 1 {
            if greater_y == second.y {
                return Direction::South;
            } else {
                return Direction::North;
            }
        }
    }
    panic!("Cannot get relative direction of non-adjacent coords");
}

fn pipe_pair_allows_passage_north_or_south(
    first_tuple: (Coords, Tile),
    second_tuple: (Coords, Tile),
) -> bool {
    let first = first_tuple.0;
    let second = second_tuple.0;
    let first_tile = first_tuple.1;
    let second_tile = second_tuple.1;

    let direction_of_second_from_first = direction_of_second_from_first(first, second);
    if direction_of_second_from_first == Direction::West {
        // first is east of second
        !(first_tile.connects_west() && second_tile.connects_east())
    } else if direction_of_second_from_first == Direction::East {
        !(first_tile.connects_east() && second_tile.connects_west())
    } else {
        false
    }
}

fn pipe_pair_allows_passage_east_or_west(
    first_tuple: (Coords, Tile),
    second_tuple: (Coords, Tile),
) -> bool {
    let first = first_tuple.0;
    let second = second_tuple.0;
    let first_tile = first_tuple.1;
    let second_tile = second_tuple.1;

    let direction_of_second_from_first = direction_of_second_from_first(first, second);
    if direction_of_second_from_first == Direction::North {
        // first is south of second
        !(first_tile.connects_north() && second_tile.connects_south())
    } else if direction_of_second_from_first == Direction::South {
        !(first_tile.connects_south() && second_tile.connects_north())
    } else {
        false
    }
}

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    let mut accessibilities: Vec<Vec<Option<TileAccessibility>>> = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let row = line_content
            .chars()
            .map(|c| Tile::from_char(c).unwrap())
            .collect::<Vec<_>>();
        let row_len = row.len();
        grid.push(row);

        let visited_row = vec![false; row_len];
        visited.push(visited_row);

        let accessibilities_row = vec![None; row_len];
        accessibilities.push(accessibilities_row);
    }

    // find starting tile
    let mut maybe_starting_tile_coords = None;
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::StartPipe {
                maybe_starting_tile_coords = Some(Coords { x, y });
                break;
            }
        }
    }

    let grid_max_col_index = grid.len() - 1;
    // assuming all rows are equal length
    let grid_max_row_index = grid[0].len() - 1;

    let mut solution_route: Option<Vec<Coords>> = None;

    let starting_tile_coords = maybe_starting_tile_coords.unwrap();
    let mut search_stack: Vec<SearchParams> = vec![SearchParams {
        route: vec![starting_tile_coords],
    }];

    while let Some(search_params) = search_stack.pop() {
        let curr_coords = search_params.route[search_params.route.len() - 1];
        visited[curr_coords.y][curr_coords.x] = true;

        // search in every possible direction
        let curr_tile = grid[curr_coords.y][curr_coords.x];
        if curr_tile.connects_north()
            && curr_coords.y > 0
            && grid[curr_coords.y - 1][curr_coords.x].connects_south()
        {
            if search_params.route.len() > 2
                && grid[curr_coords.y - 1][curr_coords.x] == Tile::StartPipe
            {
                solution_route = Some(search_params.route);
                break;
            } else if !visited[curr_coords.y - 1][curr_coords.x] {
                let mut new_route = search_params.route.clone();
                new_route.push(Coords {
                    y: curr_coords.y - 1,
                    x: curr_coords.x,
                });
                search_stack.push(SearchParams { route: new_route });
            }
        }

        if curr_tile.connects_south()
            && curr_coords.y < grid_max_col_index
            && grid[curr_coords.y + 1][curr_coords.x].connects_north()
        {
            if search_params.route.len() > 2
                && grid[curr_coords.y + 1][curr_coords.x] == Tile::StartPipe
            {
                solution_route = Some(search_params.route);
                break;
            } else if !visited[curr_coords.y + 1][curr_coords.x] {
                let mut new_route = search_params.route.clone();
                new_route.push(Coords {
                    y: curr_coords.y + 1,
                    x: curr_coords.x,
                });
                search_stack.push(SearchParams { route: new_route });
            }
        }

        if curr_tile.connects_west()
            && curr_coords.x > 0
            && grid[curr_coords.y][curr_coords.x - 1].connects_east()
        {
            if search_params.route.len() > 2
                && grid[curr_coords.y][curr_coords.x - 1] == Tile::StartPipe
            {
                solution_route = Some(search_params.route);
                break;
            } else if !visited[curr_coords.y][curr_coords.x - 1] {
                let mut new_route = search_params.route.clone();
                new_route.push(Coords {
                    y: curr_coords.y,
                    x: curr_coords.x - 1,
                });
                search_stack.push(SearchParams { route: new_route });
            }
        }

        if curr_tile.connects_east()
            && curr_coords.x < grid_max_row_index
            && grid[curr_coords.y][curr_coords.x + 1].connects_west()
        {
            if search_params.route.len() > 2
                && grid[curr_coords.y][curr_coords.x + 1] == Tile::StartPipe
            {
                solution_route = Some(search_params.route);
                break;
            } else if !visited[curr_coords.y][curr_coords.x + 1] {
                let mut new_route = search_params.route.clone();
                new_route.push(Coords {
                    y: curr_coords.y,
                    x: curr_coords.x + 1,
                });
                search_stack.push(SearchParams { route: new_route });
            }
        }
    }

    let solution_route_unwrapped = solution_route.unwrap();
    let farthest_step = solution_route_unwrapped.len() / 2;
    println!("Part 1 solution: {farthest_step}");

    let main_loop_coords: HashSet<Coords> = HashSet::from_iter(solution_route_unwrapped);

    // figure out what pipe type the StartPipe must actually be
    let start_pipe_connects_west = starting_tile_coords.x > 0
        && grid[starting_tile_coords.y][starting_tile_coords.x - 1].connects_east();
    let start_pipe_connects_east = starting_tile_coords.x < grid_max_row_index
        && grid[starting_tile_coords.y][starting_tile_coords.x + 1].connects_west();
    let start_pipe_connects_north = starting_tile_coords.y > 0
        && grid[starting_tile_coords.y - 1][starting_tile_coords.x].connects_south();
    let start_pipe_connects_south = starting_tile_coords.y < grid_max_col_index
        && grid[starting_tile_coords.y + 1][starting_tile_coords.x].connects_north();
    let starting_tile_type = [
        Tile::VerticalPipe,
        Tile::HorizontalPipe,
        Tile::NorthEastPipe,
        Tile::NorthWestPipe,
        Tile::SouthEastPipe,
        Tile::SouthWestPipe,
    ]
    .iter()
    .find(|tile_type| {
        tile_type.connects_south() == start_pipe_connects_south
            && tile_type.connects_north() == start_pipe_connects_north
            && tile_type.connects_west() == start_pipe_connects_west
            && tile_type.connects_east() == start_pipe_connects_east
    })
    .unwrap();
    // substitute in the starting tile's real type
    grid[starting_tile_coords.y][starting_tile_coords.x] = *starting_tile_type;

    // set relevant accessibility for all main-loop pipe tiles
    for coords in main_loop_coords.iter() {
        accessibilities[coords.y][coords.x] = Some(TileAccessibility::MainLoopPipe);
    }

    // replace all freestanding pipe pieces with ground for ease of search
    for y in 0..(grid_max_col_index + 1) {
        for x in 0..(grid_max_row_index + 1) {
            if !main_loop_coords.contains(&Coords { x, y }) {
                grid[y][x] = Tile::Ground;
            }
        }
    }

    println!(
        "{}",
        display_grid_with_known_accessibilities(&grid, &accessibilities)
    );

    for y in 0..(grid_max_col_index + 1) {
        for x in 0..(grid_max_row_index + 1) {
            // we may have already determined this tile's type when looking ahead
            if accessibilities[y][x].is_none() {
                if grid[y][x] == Tile::Ground {
                    // determine whether this and adjacent ground tiles are enclosed
                    let mut current_search_connects_to_outside = false;
                    let mut current_searched_ground_coords: HashSet<Coords> = HashSet::new();
                    let mut current_searched_pipe_coords: HashSet<(Coords, Coords)> =
                        HashSet::new();
                    let mut search_stack = vec![CoordsMaybePipe {
                        coords: Coords { x, y },
                        additional_pipe_coords: None,
                    }];
                    while let Some(coords_maybe_pipe) = search_stack.pop() {
                        let coords = coords_maybe_pipe.coords;
                        let tile = grid[coords.y][coords.x];
                        if tile == Tile::Ground {
                            if current_searched_ground_coords.contains(&coords) {
                                continue;
                            }
                            current_searched_ground_coords.insert(coords);
                        } else {
                            let other_pipe = coords_maybe_pipe.additional_pipe_coords.unwrap();
                            if current_searched_pipe_coords.contains(&(coords, other_pipe))
                                || current_searched_pipe_coords.contains(&(other_pipe, coords))
                            {
                                continue;
                            }
                            current_searched_pipe_coords.insert((coords, other_pipe));
                        }

                        let north: Option<(Coords, Tile)> = if coords.y > 0 {
                            let coords = Coords {
                                x: coords.x,
                                y: coords.y - 1,
                            };
                            Some((coords, grid[coords.y][coords.x]))
                        } else {
                            None
                        };
                        let northwest: Option<(Coords, Tile)> = if coords.y > 0 && coords.x > 0 {
                            let coords = Coords {
                                x: coords.x - 1,
                                y: coords.y - 1,
                            };
                            Some((coords, grid[coords.y][coords.x]))
                        } else {
                            None
                        };
                        let northeast: Option<(Coords, Tile)> =
                            if coords.y > 0 && coords.x < grid_max_row_index {
                                let coords = Coords {
                                    x: coords.x + 1,
                                    y: coords.y - 1,
                                };
                                Some((coords, grid[coords.y][coords.x]))
                            } else {
                                None
                            };
                        let east: Option<(Coords, Tile)> = if coords.x < grid_max_row_index {
                            let coords = Coords {
                                x: coords.x + 1,
                                y: coords.y,
                            };
                            Some((coords, grid[coords.y][coords.x]))
                        } else {
                            None
                        };
                        let west: Option<(Coords, Tile)> = if coords.x > 0 {
                            let coords = Coords {
                                x: coords.x - 1,
                                y: coords.y,
                            };
                            Some((coords, grid[coords.y][coords.x]))
                        } else {
                            None
                        };
                        let south: Option<(Coords, Tile)> = if coords.y < grid_max_col_index {
                            let coords = Coords {
                                x: coords.x,
                                y: coords.y + 1,
                            };
                            Some((coords, grid[coords.y][coords.x]))
                        } else {
                            None
                        };
                        let southwest: Option<(Coords, Tile)> =
                            if coords.y < grid_max_col_index && coords.x > 0 {
                                let coords = Coords {
                                    x: coords.x - 1,
                                    y: coords.y + 1,
                                };
                                Some((coords, grid[coords.y][coords.x]))
                            } else {
                                None
                            };
                        let southeast: Option<(Coords, Tile)> =
                            if coords.y < grid_max_col_index && coords.x < grid_max_row_index {
                                let coords = Coords {
                                    x: coords.x + 1,
                                    y: coords.y + 1,
                                };
                                Some((coords, grid[coords.y][coords.x]))
                            } else {
                                None
                            };
                        let adjacent_tiles = vec![
                            northwest, north, northeast, east, southeast, south, southwest, west,
                        ];

                        let defined_adjacent_tiles = adjacent_tiles
                            .iter()
                            .filter(|i| i.is_some())
                            .map(|i| i.unwrap())
                            .collect::<Vec<_>>();

                        // starting from ground tile
                        if tile == Tile::Ground {
                            // first: detect whether we've reached the edge of the map
                            if north.is_none()
                                || south.is_none()
                                || east.is_none()
                                || west.is_none()
                            {
                                current_search_connects_to_outside = true;
                            }
                            // case 1: from ground tile to ground tile
                            for adjacent in defined_adjacent_tiles {
                                if adjacent.1 == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: adjacent.0,
                                        additional_pipe_coords: None,
                                    })
                                }
                            }
                            // case 2: from ground tile to pipe gap
                            for pair in adjacent_tiles.windows(2) {
                                if allows_passage_between(pair[0], pair[1]) {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: pair[0].unwrap().0,
                                        additional_pipe_coords: Some(pair[1].unwrap().0),
                                    })
                                }
                            }
                        }

                        // starting from a pipe pair that we're actively squeezed between
                        if tile != Tile::Ground {
                            let paired_pipe_coords =
                                coords_maybe_pipe.additional_pipe_coords.unwrap();
                            let paired_pipe_tile = grid[paired_pipe_coords.y][paired_pipe_coords.x];
                            let paired_pipe = (paired_pipe_coords, paired_pipe_tile);
                            // first: detect whether we've reached the edge of the map
                            if ((north.is_none() || south.is_none())
                                && pipe_pair_allows_passage_north_or_south(
                                    (coords, tile),
                                    paired_pipe,
                                ))
                                || ((west.is_none() || east.is_none())
                                    && pipe_pair_allows_passage_east_or_west(
                                        (coords, tile),
                                        paired_pipe,
                                    ))
                            {
                                current_search_connects_to_outside = true;
                            }
                            let direction_of_adj_pipe_from_main =
                                direction_of_second_from_first(coords, paired_pipe_coords);
                            if pipe_pair_allows_passage_north_or_south((coords, tile), paired_pipe)
                            {
                                // case 3a: from pipe pair to ground tiles, north-south
                                let mut eligible_adjacents = vec![north, south];
                                if direction_of_adj_pipe_from_main == Direction::East {
                                    eligible_adjacents.push(northeast);
                                    eligible_adjacents.push(southeast);
                                } else if direction_of_adj_pipe_from_main == Direction::West {
                                    eligible_adjacents.push(northwest);
                                    eligible_adjacents.push(southwest);
                                }
                                let eligible_adj_ground_coords = eligible_adjacents
                                    .iter()
                                    .filter(|maybe_tup| {
                                        maybe_tup.is_some() && maybe_tup.unwrap().1 == Tile::Ground
                                    })
                                    .map(|maybe_tup| maybe_tup.unwrap().0)
                                    .collect::<Vec<_>>();
                                for adj_ground_coords in eligible_adj_ground_coords {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: adj_ground_coords,
                                        additional_pipe_coords: None,
                                    });
                                }
                                // case 4a: from pipe pair to pipe pair, originating north-south
                                let possible_adj_pipe_pairs =
                                    if direction_of_adj_pipe_from_main == Direction::East {
                                        vec![
                                            (north, northeast),
                                            (Some((coords, tile)), north), // 90-degree turn
                                            (northeast, Some(paired_pipe)), // 90-degree turn
                                            (south, southeast),
                                            (Some((coords, tile)), south), // 90-degree turn
                                            (southeast, Some(paired_pipe)), // 90-degree turn
                                        ]
                                    } else if direction_of_adj_pipe_from_main == Direction::West {
                                        vec![
                                            (northwest, north),
                                            (Some(paired_pipe), northwest), // 90-degree turn
                                            (Some((coords, tile)), north),  // 90-degree turn
                                            (southwest, south),
                                            (Some(paired_pipe), southwest), // 90-degree turn
                                            (Some((coords, tile)), south),  // 90-degree turn
                                        ]
                                    } else {
                                        Vec::new()
                                    };
                                let filtered_pairs =
                                    possible_adj_pipe_pairs.iter().filter(|(first, second)| {
                                        allows_passage_between(*first, *second)
                                    });
                                for pair in filtered_pairs {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: pair.0.unwrap().0,
                                        additional_pipe_coords: Some(pair.1.unwrap().0),
                                    })
                                }
                            }
                            if pipe_pair_allows_passage_east_or_west((coords, tile), paired_pipe) {
                                // case 3b: from pipe pair to ground tiles, east-west
                                let mut eligible_adjacents = vec![east, west];
                                if direction_of_adj_pipe_from_main == Direction::North {
                                    eligible_adjacents.push(northeast);
                                    eligible_adjacents.push(northwest);
                                } else if direction_of_adj_pipe_from_main == Direction::South {
                                    eligible_adjacents.push(southeast);
                                    eligible_adjacents.push(southwest);
                                }
                                let eligible_adj_ground_coords = eligible_adjacents
                                    .iter()
                                    .filter(|maybe_tup| {
                                        maybe_tup.is_some() && maybe_tup.unwrap().1 == Tile::Ground
                                    })
                                    .map(|maybe_tup| maybe_tup.unwrap().0)
                                    .collect::<Vec<_>>();
                                for adj_ground_coords in eligible_adj_ground_coords {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: adj_ground_coords,
                                        additional_pipe_coords: None,
                                    });
                                }
                                // case 4b: from pipe pair to pipe pair, originating east-west
                                let possible_adj_pipe_pairs =
                                    if direction_of_adj_pipe_from_main == Direction::North {
                                        vec![
                                            (east, northeast),
                                            (Some((coords, tile)), east), // 90-degree turn
                                            (Some(paired_pipe), northeast), // 90-degree turn
                                            (west, northwest),
                                            (Some((coords, tile)), west), // 90-degree turn
                                            (Some(paired_pipe), northwest), // 90-degree turn
                                        ]
                                    } else if direction_of_adj_pipe_from_main == Direction::South {
                                        vec![
                                            (east, southeast),
                                            (Some((coords, tile)), east), // 90-degree turn
                                            (Some(paired_pipe), southeast), // 90-degree turn
                                            (west, southwest),
                                            (Some((coords, tile)), west), // 90-degree turn
                                            (Some(paired_pipe), southwest), // 90-degree turn
                                        ]
                                    } else {
                                        Vec::new()
                                    };
                                let filtered_pairs =
                                    possible_adj_pipe_pairs.iter().filter(|(first, second)| {
                                        allows_passage_between(*first, *second)
                                    });
                                for pair in filtered_pairs {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: pair.0.unwrap().0,
                                        additional_pipe_coords: Some(pair.1.unwrap().0),
                                    })
                                }
                            }
                        }
                    }

                    // Update 'accessibilities' grid based on search results
                    let searched_ground_type = if current_search_connects_to_outside {
                        TileAccessibility::UnenclosedGround
                    } else {
                        TileAccessibility::MainLoopEnclosedGround
                    };
                    for coords in current_searched_ground_coords {
                        accessibilities[coords.y][coords.x] = Some(searched_ground_type);
                    }
                }
            }
        }
    }

    println!(
        "{}",
        display_grid_with_known_accessibilities(&grid, &accessibilities)
    );

    let num_main_loop_enclosed_ground_tiles = accessibilities
        .iter()
        .map(|row| {
            row.iter()
                .filter(|acc| **acc == Some(TileAccessibility::MainLoopEnclosedGround))
                .count()
        })
        .sum::<usize>();
    println!("Part 2 solution: {num_main_loop_enclosed_ground_tiles}");
}
