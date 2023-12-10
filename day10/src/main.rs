use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct SearchParams {
    route: Vec<Coords>,
}

#[derive(Debug, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
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

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
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
    }

    // find starting tile
    let mut starting_tile_coords = None;
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::StartPipe {
                starting_tile_coords = Some(Coords { x, y });
                break;
            }
        }
    }

    let grid_max_col_index = grid.len() - 1;
    // assuming all rows are equal length
    let grid_max_row_index = grid[0].len() - 1;

    let mut solution_route: Option<Vec<Coords>> = None;

    let mut search_stack: Vec<SearchParams> = vec![SearchParams {
        route: vec![starting_tile_coords.unwrap()],
    }];

    let complete_route = while let Some(search_params) = search_stack.pop() {
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
    };

    let solution_route_unwrapped = solution_route.unwrap();
    let farthest_step = solution_route_unwrapped.len() / 2;
    println!("Part 1 solution: {farthest_step}");
}
