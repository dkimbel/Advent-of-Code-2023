use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Condition {
    Damaged,
    Operational,
    Unknown,
}

impl Condition {
    fn from_char(c: char) -> Condition {
        match c {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Cannot parse {} as Condition", c),
        }
    }
}

fn num_valid_arrangements(conditions: &[Condition], contiguous_damaged_counts: &[u32]) -> usize {
    // println!("{:?} {:?}", conditions, contiguous_damaged_counts);
    // Base cases
    if contiguous_damaged_counts.is_empty() {
        if conditions.iter().any(|c| *c == Condition::Damaged) {
            // we failed to account for at least one damaged tile
            return 0;
        } else {
            // success!
            // println!("Returning 1!");
            return 1;
        }
    }

    // Heuristic / base case: there aren't enough tiles left for success to be possible
    let num_conditions_required_after_first = contiguous_damaged_counts[1..]
        .iter()
        .map(|n| n + 1) // + 1 represents the one tile we'd need to break up every damage streak
        .sum::<u32>() as usize;
    let num_remaining_conditions_required =
        contiguous_damaged_counts[0] as usize + num_conditions_required_after_first;
    if conditions.len() < num_remaining_conditions_required {
        return 0;
    }

    // Work left to right, only ever making a valid match
    let needed_damaged_count = contiguous_damaged_counts[0] as usize;
    // find next stretch of tiles that can accommodate damaged_count; stop as soon as we've found
    // a viable match
    let mut i = 0;
    let mut maybe_streak_start_i = None;
    let mut streak_len: usize = 0;
    let mut streak_has_known_damaged = false;
    let mut must_claim_extra_tile = false;
    let mut still_check_streak_incremented_by_one_from = None;
    // TODO make everything work with this 'after first' heuristic
    // the removal of num_conditions_required_after_first is particularly effective since we
    // stripped extra 'operational' tiles during parsing
    let max_i_to_consider = conditions.len() - num_conditions_required_after_first - 1;
    // println!("Considering {:?}", &conditions[0..max_i_to_consider + 1]);
    while i <= max_i_to_consider {
        let condition = conditions[i];
        if condition == Condition::Operational {
            if streak_len >= needed_damaged_count {
                // arguably we should increment streak_len here, but we'll definitely move past
                // this 'operational' tile next iteration anyway
                break;
            } else {
                // Here we _potentially_ reset, figuring that we can safely skip the whole streak
                // we were just looking at (if any). This effectively means swapping in '.' for
                // every character of the streak, because the steak was too short. HOWEVER, this is
                // only safe to do if the streak was made up entirely of '?' -- if it had any '#'
                // yet wasn't long enough to accommodate the next streak we have to match, it would
                // ruin everything, and we need to just return 0 (failure case).
                maybe_streak_start_i = None;
                streak_len = 0;
                if streak_has_known_damaged {
                    still_check_streak_incremented_by_one_from = None;
                    break;
                }
            }
        } else {
            if condition == Condition::Damaged {
                streak_has_known_damaged = true;
            }
            streak_len += 1;
            if maybe_streak_start_i == None {
                maybe_streak_start_i = Some(i)
            }
            if condition == Condition::Unknown && streak_len == needed_damaged_count + 1 {
                // effectively transform the last character of the streak from '?' to '.',
                // successfully ending it
                break;
            } else if condition == Condition::Damaged && streak_len > needed_damaged_count {
                // there were too damaged tiles in this streak, and we weren't able to split them
                // apart in a way that meets requirements
                still_check_streak_incremented_by_one_from = maybe_streak_start_i;
                maybe_streak_start_i = None; // indicate failure
                break;
            }
        }
        if i == max_i_to_consider {
            // We've reached the end of section we can reasonably consider. If we've found a
            // streak of the right size AND this is a safe stopping place, return success. "A
            // safe stopping place" is either the end of the whole conditions list, or the
            // end of our "reasonable consideration" section if the next tile after it is
            // Operational.
            if maybe_streak_start_i.is_some() && streak_len >= needed_damaged_count {
                if i == conditions.len() - 1 {
                    // we're at end of whole list
                    break;
                } else {
                    if conditions[max_i_to_consider + 1] != Condition::Damaged {
                        // next tile would actually give us a safe stopping place, but we have
                        // to consume it
                        must_claim_extra_tile = true;
                        break;
                    }
                }
            }
            // Failure case
            maybe_streak_start_i = None; // indicate failure
            break;
        }
        i += 1;
    }

    match maybe_streak_start_i {
        None => {
            if still_check_streak_incremented_by_one_from.is_some() {
                let initial_condition = &conditions[0];
                if initial_condition != &Condition::Damaged {
                    return num_valid_arrangements(&conditions[1..], &contiguous_damaged_counts);
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        Some(streak_start_i) => {
            // take up to two actions:
            // 1. claim the streak we found above (always)
            // 2. swap in '.' for the first '?', IF the '?' is the very first character (else this
            //    would create an unwanted initial streak of damaged)
            let maybe_extra_tile = if must_claim_extra_tile { 1 } else { 0 };
            let streak_end_i = streak_start_i + streak_len + maybe_extra_tile;
            let num_arrangements_after_streak = num_valid_arrangements(
                &conditions[streak_end_i..],
                &contiguous_damaged_counts[1..],
            );
            let first_condition = &conditions[streak_start_i];
            if first_condition == &Condition::Unknown {
                return num_arrangements_after_streak
                    + num_valid_arrangements(
                        &conditions[streak_start_i + 1..],
                        &contiguous_damaged_counts,
                    );
            } else {
                return num_arrangements_after_streak;
            }
        }
    }
}

#[derive(Debug)]
struct Row {
    conditions: Vec<Condition>,
    contiguous_damaged_counts: Vec<u32>,
}

fn main() {
    let file = File::open("resources/sample_6").unwrap();
    let reader = BufReader::new(file);

    let mut rows: Vec<Row> = Vec::new();
    let mut expanded_rows: Vec<Row> = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let split = line_content.split_whitespace().collect::<Vec<_>>();
        let conditions = split[0]
            .chars()
            .map(Condition::from_char)
            .collect::<Vec<_>>();
        let contiguous_damaged_counts = split[1]
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        rows.push(Row {
            conditions: conditions.clone(),
            contiguous_damaged_counts: contiguous_damaged_counts.clone(),
        });
        // janky 'intersperse'
        let mut expanded_conditions: Vec<Condition> = Vec::new();
        for i in 0..5 {
            expanded_conditions.extend(&conditions);
            if i < 4 {
                expanded_conditions.push(Condition::Unknown);
            }
        }
        let mut pared_expanded_conditions: Vec<Condition> = Vec::new();
        let mut last_ele_was_operational = false;
        for (i, cond) in expanded_conditions.iter().enumerate() {
            let eligible_to_strip_operational =
                i == 0 || i == expanded_conditions.len() - 1 || last_ele_was_operational;
            if eligible_to_strip_operational && *cond == Condition::Operational {
                last_ele_was_operational = true;
                continue;
            } else {
                if *cond == Condition::Operational {
                    last_ele_was_operational = true;
                } else {
                    last_ele_was_operational = false;
                }
                pared_expanded_conditions.push(*cond);
            }
        }
        let mut expanded_damage_counts: Vec<u32> = Vec::new();
        for _ in 0..5 {
            expanded_damage_counts.extend(&contiguous_damaged_counts);
        }
        expanded_rows.push(Row {
            conditions: pared_expanded_conditions,
            contiguous_damaged_counts: expanded_damage_counts,
        });
    }

    let total_valid_arrangements = rows
        .iter()
        // .map(|row| num_valid_arrangements(&row.conditions, &row.contiguous_damaged_counts))
        .map(|row| {
            // let num_arrangements = Row::num_valid_arrangements_brute_force(row);
            let num_arrangements =
                num_valid_arrangements(&row.conditions, &row.contiguous_damaged_counts);
            // println!("{:?} {}", row, num_arrangements);
            num_arrangements
        })
        .sum::<usize>();
    println!("Part 1 solution: {total_valid_arrangements}");

    // let total_valid_expanded_arrangements = expanded_rows
    //     .iter()
    //     .enumerate()
    //     // .map(|row| num_valid_arrangements(&row.conditions, &row.contiguous_damaged_counts))
    //     .map(|(i, row)| {
    //         // let num_arrangements = Row::num_valid_arrangements_brute_force(row);
    //         let num_arrangements =
    //             num_valid_arrangements(&row.conditions, &row.contiguous_damaged_counts);
    //         println!("{} {}", i, num_arrangements);
    //         num_arrangements
    //     })
    //     .sum::<usize>();
    // println!("Part 2 solution: {total_valid_expanded_arrangements}");
}
