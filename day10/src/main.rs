use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TileAccessibility {
    UnenclosedGround,
    MainLoopEnclosedGround,
    OtherEnclosedGround,
    Pipe,
}

#[derive(Debug)]
struct SearchParams {
    route: Vec<Coords>,
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
                        None => tile.to_char(),
                        Some(TileAccessibility::UnenclosedGround) => 'O',
                        Some(TileAccessibility::MainLoopEnclosedGround) => 'I',
                        Some(TileAccessibility::OtherEnclosedGround) => 'Z',
                        _ => panic!("Could not print grid"),
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

fn pipe_pairs_align_vertically(
    first_pair: (Coords, Coords),
    second_pair: (Coords, Coords),
) -> bool {
    let same_x_values = (first_pair.0.x == second_pair.0.x && first_pair.1.x == second_pair.1.x)
        || (first_pair.0.x == second_pair.1.x && first_pair.1.x == second_pair.0.x);
    let internally_consistent_y_values =
        first_pair.0.y == first_pair.1.y && second_pair.0.y == second_pair.1.y;
    let higher_y = std::cmp::max(first_pair.0.y, second_pair.0.y);
    let lesser_y = std::cmp::min(first_pair.0.y, second_pair.0.y);
    let y_values_within_one = higher_y - lesser_y == 1;
    same_x_values && internally_consistent_y_values && y_values_within_one
}

fn pipe_pairs_align_horizontally(
    first_pair: (Coords, Coords),
    second_pair: (Coords, Coords),
) -> bool {
    let same_y_values = (first_pair.0.y == second_pair.0.y && first_pair.1.y == second_pair.1.y)
        || (first_pair.0.y == second_pair.1.y && first_pair.1.y == second_pair.0.y);
    let internally_consistent_x_values =
        first_pair.0.x == first_pair.1.x && second_pair.0.x == second_pair.1.x;
    let higher_x = std::cmp::max(first_pair.0.x, second_pair.0.x);
    let lesser_x = std::cmp::min(first_pair.0.x, second_pair.0.x);
    let x_values_within_one = higher_x - lesser_x == 1;
    same_y_values && internally_consistent_x_values && x_values_within_one
}

fn pipe_pair_allows_passage_north_or_south(
    first: Coords,
    second: Coords,
    grid: &Vec<Vec<Tile>>,
) -> bool {
    if first.y != second.y {
        // invalid pair, forbid passage
        return false;
    }
    let greater = std::cmp::max(first.x, second.x);
    let lesser = std::cmp::min(first.x, second.x);
    if greater - lesser != 1 {
        // invalid pair, forbid passage
        return false;
    }
    let first_tile = grid[first.y][first.x];
    let second_tile = grid[second.y][second.x];
    if first.x > second.x {
        // first is east of second
        !(first_tile.connects_west() && second_tile.connects_east())
    } else {
        !(first_tile.connects_east() && second_tile.connects_west())
    }
}

fn pipe_pair_allows_passage_east_or_west(
    first: Coords,
    second: Coords,
    grid: &Vec<Vec<Tile>>,
) -> bool {
    if first.x != second.x {
        // invalid pair, forbid passage
        return false;
    }
    let greater = std::cmp::max(first.y, second.y);
    let lesser = std::cmp::min(first.y, second.y);
    if greater - lesser != 1 {
        // invalid pair, forbid passage
        return false;
    }
    let first_tile = grid[first.y][first.x];
    let second_tile = grid[second.y][second.x];
    if first.y > second.y {
        // first is south of second
        !(first_tile.connects_north() && second_tile.connects_south())
    } else {
        !(first_tile.connects_south() && second_tile.connects_north())
    }
}

fn main() {
    let file = File::open("resources/sample_5").unwrap();
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
                    let mut current_search_was_impeded_by_main_loop = false;
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

                        println!("Checking coords {:?}", coords);

                        let mut northwest_pipe: Option<(Coords, Tile)> = None;
                        let mut northeast_pipe: Option<(Coords, Tile)> = None;
                        let mut southwest_pipe: Option<(Coords, Tile)> = None;
                        let mut southeast_pipe: Option<(Coords, Tile)> = None;

                        if coords.y == 0 {
                            // the outside is just north of us!
                            if tile == Tile::Ground {
                                current_search_connects_to_outside = true;
                            } else {
                                if pipe_pair_allows_passage_north_or_south(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                ) {
                                    current_search_connects_to_outside = true;
                                } else if !current_search_was_impeded_by_main_loop
                                    && main_loop_coords.contains(&coords)
                                {
                                    current_search_was_impeded_by_main_loop = true;
                                }
                            }
                        } else {
                            // check northwest
                            if coords.x > 0 {
                                let target_coords = Coords {
                                    x: coords.x - 1,
                                    y: coords.y - 1,
                                };
                                let target_tile = grid[target_coords.y][target_coords.x];
                                // can only move diagonally from ground to ground
                                if target_tile == Tile::Ground && tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    northwest_pipe = Some((target_coords, target_tile));
                                }
                            }
                            // check northeast
                            if coords.x < grid_max_row_index {
                                let target_coords = Coords {
                                    x: coords.x + 1,
                                    y: coords.y - 1,
                                };
                                let target_tile = grid[target_coords.y][target_coords.x];
                                // can only move diagonally from ground to ground
                                if target_tile == Tile::Ground && tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    northeast_pipe = Some((target_coords, target_tile));
                                }
                            }
                            // check due north
                            let target_coords = Coords {
                                x: coords.x,
                                y: coords.y - 1,
                            };
                            let target_tile = grid[target_coords.y][target_coords.x];
                            // ensure we either aren't squeezed between pipes at all, or are
                            // squeezed between pipes that allow passage north
                            if tile == Tile::Ground
                                || pipe_pair_allows_passage_north_or_south(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                )
                            {
                                if target_tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    // we are trying to enter from the south
                                    if northwest_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_vertically(
                                                (target_coords, northwest_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_north_or_south(
                                            target_coords,
                                            northwest_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    northwest_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }

                                    if northeast_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_vertically(
                                                (target_coords, northeast_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_north_or_south(
                                            target_coords,
                                            northeast_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    northeast_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if coords.y == grid_max_col_index {
                            // the outside is just south of us!
                            if tile == Tile::Ground {
                                current_search_connects_to_outside = true;
                            } else {
                                if pipe_pair_allows_passage_north_or_south(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                ) {
                                    current_search_connects_to_outside = true;
                                } else if !current_search_was_impeded_by_main_loop
                                    && main_loop_coords.contains(&coords)
                                {
                                    current_search_was_impeded_by_main_loop = true;
                                }
                            }
                        } else {
                            // check southwest
                            if coords.x > 0 {
                                let target_coords = Coords {
                                    x: coords.x - 1,
                                    y: coords.y + 1,
                                };
                                let target_tile = grid[target_coords.y][target_coords.x];
                                // can only move diagonally from ground to ground
                                if target_tile == Tile::Ground && tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    southwest_pipe = Some((target_coords, target_tile));
                                }
                            }
                            // check southeast
                            if coords.x < grid_max_row_index {
                                let target_coords = Coords {
                                    x: coords.x + 1,
                                    y: coords.y + 1,
                                };
                                let target_tile = grid[target_coords.y][target_coords.x];
                                // can only move diagonally from ground to ground
                                if target_tile == Tile::Ground && tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    southeast_pipe = Some((target_coords, target_tile));
                                }
                            }
                            // check due south
                            let target_coords = Coords {
                                x: coords.x,
                                y: coords.y + 1,
                            };
                            let target_tile = grid[target_coords.y][target_coords.x];
                            // ensure we either aren't squeezed between pipes at all, or are
                            // squeezed between pipes that allow passage south
                            if tile == Tile::Ground
                                || pipe_pair_allows_passage_north_or_south(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                )
                            {
                                if target_tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    // we are trying to enter from the north
                                    if southwest_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_vertically(
                                                (target_coords, southwest_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_north_or_south(
                                            target_coords,
                                            southwest_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    southwest_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }

                                    if southeast_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_vertically(
                                                (target_coords, southeast_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_north_or_south(
                                            target_coords,
                                            southeast_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    southeast_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if coords.x == 0 {
                            // the outside is just west of us!
                            if tile == Tile::Ground {
                                current_search_connects_to_outside = true;
                            } else {
                                if pipe_pair_allows_passage_east_or_west(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                ) {
                                    current_search_connects_to_outside = true;
                                } else if !current_search_was_impeded_by_main_loop
                                    && main_loop_coords.contains(&coords)
                                {
                                    current_search_was_impeded_by_main_loop = true;
                                }
                            }
                        } else {
                            // check due west
                            let target_coords = Coords {
                                x: coords.x - 1,
                                y: coords.y,
                            };
                            let target_tile = grid[target_coords.y][target_coords.x];
                            // ensure we either aren't squeezed between pipes at all, or are
                            // squeezed between pipes that allow passage west
                            if tile == Tile::Ground
                                || pipe_pair_allows_passage_east_or_west(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                )
                            {
                                if target_tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    // we are trying to enter from the east
                                    if southwest_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_horizontally(
                                                (target_coords, southwest_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_east_or_west(
                                            target_coords,
                                            southwest_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    southwest_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }

                                    if northwest_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_horizontally(
                                                (target_coords, northwest_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_east_or_west(
                                            target_coords,
                                            northwest_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    northwest_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if coords.x == grid_max_row_index {
                            // the outside is just east of us!
                            if tile == Tile::Ground {
                                current_search_connects_to_outside = true;
                            } else {
                                if pipe_pair_allows_passage_east_or_west(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                ) {
                                    current_search_connects_to_outside = true;
                                } else if !current_search_was_impeded_by_main_loop
                                    && main_loop_coords.contains(&coords)
                                {
                                    current_search_was_impeded_by_main_loop = true;
                                }
                            }
                        } else {
                            // check due east
                            let target_coords = Coords {
                                x: coords.x + 1,
                                y: coords.y,
                            };
                            let target_tile = grid[target_coords.y][target_coords.x];
                            // ensure we either aren't squeezed between pipes at all, or are
                            // squeezed between pipes that allow passage east
                            if tile == Tile::Ground
                                || pipe_pair_allows_passage_east_or_west(
                                    coords,
                                    coords_maybe_pipe.additional_pipe_coords.unwrap(),
                                    &grid,
                                )
                            {
                                if target_tile == Tile::Ground {
                                    search_stack.push(CoordsMaybePipe {
                                        coords: target_coords,
                                        additional_pipe_coords: None,
                                    });
                                } else {
                                    // we are trying to enter from the west
                                    if southeast_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_horizontally(
                                                (target_coords, southeast_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_east_or_west(
                                            target_coords,
                                            southeast_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    southeast_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }

                                    if northeast_pipe.is_some()
                                        && (tile == Tile::Ground
                                            || pipe_pairs_align_horizontally(
                                                (target_coords, northeast_pipe.unwrap().0),
                                                (
                                                    coords,
                                                    coords_maybe_pipe
                                                        .additional_pipe_coords
                                                        .unwrap(),
                                                ),
                                            ))
                                    {
                                        if pipe_pair_allows_passage_east_or_west(
                                            target_coords,
                                            northeast_pipe.unwrap().0,
                                            &grid,
                                        ) {
                                            // successfully squeezing between pipes
                                            search_stack.push(CoordsMaybePipe {
                                                coords: target_coords,
                                                additional_pipe_coords: Some(
                                                    northeast_pipe.unwrap().0,
                                                ),
                                            });
                                        } else {
                                            // blocked!
                                            if !current_search_was_impeded_by_main_loop
                                                && main_loop_coords.contains(&target_coords)
                                            {
                                                current_search_was_impeded_by_main_loop = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Update 'accessibilities' grid based on search results
                    let searched_ground_type = if current_search_connects_to_outside {
                        TileAccessibility::UnenclosedGround
                    } else if current_search_was_impeded_by_main_loop {
                        TileAccessibility::MainLoopEnclosedGround
                    } else {
                        TileAccessibility::OtherEnclosedGround
                    };
                    for coords in current_searched_ground_coords {
                        accessibilities[coords.y][coords.x] = Some(searched_ground_type);
                    }
                } else {
                    accessibilities[y][x] = Some(TileAccessibility::Pipe);
                }
                println!(
                    "{}",
                    display_grid_with_known_accessibilities(&grid, &accessibilities)
                );
            }
        }
    }

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
