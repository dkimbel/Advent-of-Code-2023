use std::fs;

fn score_grid_part_2(grid: &Vec<Vec<char>>) -> i64 {
    // first, check for reflected row
    let mut row_after: i64 = 0;
    let mut maybe_reflected_row_after = None;
    let grid_len = grid.len() as i64;
    while maybe_reflected_row_after.is_none() && row_after < grid_len {
        // work both up and down from row_after as long as we can
        let mut diff: i64 = 1;
        let mut mismatches: i64 = 0;
        while row_after - diff >= 0 && (row_after + diff - 1) < grid_len {
            let row_above = &grid[(row_after - diff) as usize];
            let row_below = &grid[(row_after + (diff - 1)) as usize];
            for (c1, c2) in std::iter::zip(row_above, row_below) {
                if c1 != c2 {
                    mismatches += 1;
                }
            }
            diff += 1;
        }
        if mismatches == 1 {
            maybe_reflected_row_after = Some(row_after);
            break;
        }
        row_after += 1;
    }

    if maybe_reflected_row_after.is_some() {
        return maybe_reflected_row_after.unwrap() * 100;
    }

    // check for reflected column
    let grid_width = grid[0].len() as i64;
    let mut col_after: i64 = 0;
    let mut maybe_reflected_col_after = None;
    while maybe_reflected_col_after.is_none() && col_after < grid_width {
        // work both up and down from row_after as long as we can
        let mut diff: i64 = 1;
        let mut mismatches: i64 = 0;
        while col_after - diff >= 0 && (col_after + diff - 1) < grid_width {
            let i_left = (col_after - diff) as usize;
            let i_right = (col_after + diff - 1) as usize;
            let col_left: Vec<char> = grid.iter().map(|row| row[i_left]).collect::<Vec<_>>();
            let col_right: Vec<char> = grid.iter().map(|row| row[i_right]).collect::<Vec<_>>();
            for (c1, c2) in std::iter::zip(col_left, col_right) {
                if c1 != c2 {
                    mismatches += 1;
                }
            }
            diff += 1;
        }
        if mismatches == 1 {
            maybe_reflected_col_after = Some(col_after);
            break;
        }
        col_after += 1;
    }

    if maybe_reflected_col_after.is_some() {
        return maybe_reflected_col_after.unwrap();
    }
    panic!("No smudged reflection found!");
}

fn score_grid_part_1(grid: &Vec<Vec<char>>) -> i64 {
    // first, check for reflected row
    let mut row_after: i64 = 0;
    let mut maybe_reflected_row_after = None;
    let grid_len = grid.len() as i64;
    while maybe_reflected_row_after.is_none() && row_after < grid_len {
        // work both up and down from row_after as long as we can
        let mut diff: i64 = 1;
        let mut reflected = false;
        while row_after - diff >= 0 && (row_after + diff - 1) < grid_len {
            let row_above = &grid[(row_after - diff) as usize];
            let row_below = &grid[(row_after + (diff - 1)) as usize];
            if row_above == row_below {
                reflected = true;
            } else {
                reflected = false;
                break;
            }
            diff += 1;
        }
        if reflected {
            maybe_reflected_row_after = Some(row_after);
            break;
        }
        row_after += 1;
    }

    if maybe_reflected_row_after.is_some() {
        return maybe_reflected_row_after.unwrap() * 100;
    }

    // check for reflected column
    let grid_width = grid[0].len() as i64;
    let mut col_after: i64 = 0;
    let mut maybe_reflected_col_after = None;
    while maybe_reflected_col_after.is_none() && col_after < grid_width {
        // work both up and down from row_after as long as we can
        let mut diff: i64 = 1;
        let mut reflected = false;
        while col_after - diff >= 0 && (col_after + diff - 1) < grid_width {
            let i_left = (col_after - diff) as usize;
            let i_right = (col_after + diff - 1) as usize;
            let col_left: Vec<char> = grid.iter().map(|row| row[i_left]).collect::<Vec<_>>();
            let col_right: Vec<char> = grid.iter().map(|row| row[i_right]).collect::<Vec<_>>();
            if col_left == col_right {
                reflected = true;
            } else {
                reflected = false;
                break;
            }
            diff += 1;
        }
        if reflected {
            maybe_reflected_col_after = Some(col_after);
            break;
        }
        col_after += 1;
    }

    if maybe_reflected_col_after.is_some() {
        return maybe_reflected_col_after.unwrap();
    }
    panic!("No reflection found!");
}

fn main() {
    let mut part_1_score: i64 = 0;
    let mut part_2_score: i64 = 0;

    let file_content = fs::read_to_string("resources/input_1").unwrap();
    let input_chunks = file_content.split("\n\n").collect::<Vec<_>>();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for chunk in input_chunks {
        for row in chunk.lines() {
            grid.push(row.chars().collect::<Vec<char>>());
        }

        part_1_score += score_grid_part_1(&grid);
        part_2_score += score_grid_part_2(&grid);
        grid.clear();
    }

    println!("Part 1 solution: {part_1_score}");
    println!("Part 2 solution: {part_2_score}");
}
