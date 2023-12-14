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

#[derive(Debug)]
struct ContiguousConditions {
    condition: Condition,
    num: u32,
}

impl ContiguousConditions {
    fn vec_from_conditions_vec(conditions: Vec<Condition>) -> Vec<ContiguousConditions> {
        let mut contiguous_conditions = Vec::new();
        let mut curr_count = 1;
        let mut curr_condition = conditions[0];
        for (i, condition) in conditions.iter().enumerate().skip(1) {
            if *condition == curr_condition {
                curr_count += 1;
            } else {
                contiguous_conditions.push(ContiguousConditions {
                    condition: curr_condition,
                    num: curr_count,
                });
                curr_count = 1;
                curr_condition = *condition;
            }

            if i == conditions.len() - 1 {
                // we've reached the last condition
                contiguous_conditions.push(ContiguousConditions {
                    condition: curr_condition,
                    num: curr_count,
                });
            }
        }
        contiguous_conditions
    }
}

fn num_valid_arrangements(conditions: &[Condition], contiguous_damaged_counts: &[u32]) -> usize {
    println!(
        "In fn! conditions: {:?}, contiguous_damaged_counts: {:?}",
        conditions, contiguous_damaged_counts
    );
    // Base cases (success)
    if contiguous_damaged_counts.is_empty() {
        println!("Returning 1!");
        return 1;
    }

    // Heuristic / base case: there aren't enough tiles left for success to be possible
    let num_remaining_conditions_required = contiguous_damaged_counts.iter().sum::<u32>() as usize
        + (contiguous_damaged_counts.len() - 1);
    if conditions.len() < num_remaining_conditions_required {
        println!("Returning 0");
        return 0;
    }

    // Work left to right, only ever making a valid match
    let needed_damaged_count = contiguous_damaged_counts[0] as usize;
    // find next stretch of tiles that can accommodate damaged_count; stop as soon as we've found
    // a viable match
    let mut i = 0;
    let mut maybe_streak_start_i = None;
    let mut streak_len: usize = 0;
    let mut streak_len_since_known_damaged_inclusive = 0;
    let mut streak_has_known_damaged = false;
    while i < conditions.len() {
        let condition = conditions[i];
        if condition == Condition::Operational {
            if streak_len >= needed_damaged_count {
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
                    break;
                }
            }
        } else {
            if condition == Condition::Damaged {
                streak_has_known_damaged = true;
            }
            streak_len += 1;
            if streak_has_known_damaged {
                // "inclusive" as in "this len counts that initial damaged tile"
                streak_len_since_known_damaged_inclusive += 1;
            }
            if maybe_streak_start_i == None {
                maybe_streak_start_i = Some(i)
            }
            // important that this comes after the code above, which increased the length of our
            // 'streak' -- here we may effectively transform the last character of the streak from
            // a '?' to a '.', which counts as consuming part of it
            if condition == Condition::Unknown && streak_len >= needed_damaged_count {
                break;
            } else if condition == Condition::Damaged
                && streak_len_since_known_damaged_inclusive > needed_damaged_count
            {
                // there were too damaged tiles in this streak, and we weren't able to split them
                // apart in a way that meets requirements
                break;
            }
        }
        i += 1;
    }

    match maybe_streak_start_i {
        None => return 0,
        Some(streak_start_i) => {
            // take up to two actions:
            // 1. claim the streak we found above (always)
            // 2. swap in '.' for the first '?', IF the '?' is the very first character (else this
            //    would create an unwanted initial streak of damaged)
            let streak_end_i = streak_start_i + streak_len;
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

impl Row {
    fn num_valid_arrangements_brute_force(&self) -> usize {
        let unknown_condition_indices: Vec<usize> = self
            .conditions
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == Condition::Unknown)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let num_unknown: usize = unknown_condition_indices.len();
        if num_unknown == 0 {
            return 1;
        }

        // brute force: test every possibility
        let two: u32 = 2;
        let num_possible_arrangements = two.pow(num_unknown as u32);
        let mut all_possible_arrangements: Vec<Vec<Condition>> =
            vec![self.conditions.clone(); num_possible_arrangements as usize];
        for (index_num, unknown_i) in unknown_condition_indices.iter().enumerate() {
            let change_every = two.pow(index_num as u32) as usize;
            let mut curr_condition = Condition::Damaged;
            for (arrangement_num, possible_arrangement_i) in
                (0..num_possible_arrangements as usize).enumerate()
            {
                all_possible_arrangements[possible_arrangement_i][*unknown_i] = curr_condition;
                let division_safe_arrangement_num = arrangement_num + 1;
                if division_safe_arrangement_num % change_every == 0 {
                    curr_condition = match curr_condition {
                        Condition::Damaged => Condition::Operational,
                        Condition::Operational => Condition::Damaged,
                        Condition::Unknown => panic!("Unknown condition not valid here"),
                    }
                }
            }
        }
        let mut num_valid_arrangements = 0;
        for arrangement in all_possible_arrangements {
            let contiguous = ContiguousConditions::vec_from_conditions_vec(arrangement);
            let arrangement_damaged_counts = contiguous
                .iter()
                .filter(|cc| cc.condition == Condition::Damaged)
                .map(|cc| cc.num)
                .collect::<Vec<_>>();
            if arrangement_damaged_counts == self.contiguous_damaged_counts {
                num_valid_arrangements += 1;
            }
        }
        num_valid_arrangements
    }
}

fn main() {
    let file = File::open("resources/sample_2").unwrap();
    let reader = BufReader::new(file);

    let mut rows: Vec<Row> = Vec::new();
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
            conditions,
            contiguous_damaged_counts,
        });
    }

    // let total_valid_arrangements = rows
    //     .iter()
    //     .map(|row| num_valid_arrangements(&row.conditions, &row.contiguous_damaged_counts))
    //     .sum::<usize>();
    let valid_arrangements = rows
        .iter()
        .map(|row| num_valid_arrangements(&row.conditions, &row.contiguous_damaged_counts))
        .collect::<Vec<_>>();
    dbg!(&valid_arrangements);
    let total_valid_arrangements = valid_arrangements.iter().sum::<usize>();
    println!("Part 2 solution: {total_valid_arrangements}");
}
