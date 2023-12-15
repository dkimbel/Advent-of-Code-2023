use std::fs;

fn apply_hash_algorithm(input: &str) -> u8 {
    let mut result: u64 = 0;
    for c in input.chars() {
        let ascii_code = c as u8;
        result += ascii_code as u64;
        result *= 17;
        result %= 256;
    }
    result as u8
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

enum InstructionType {
    Remove { label: String },
    Replace { label: String, focal_length: u8 },
}

impl InstructionType {
    fn label(&self) -> &str {
        match self {
            InstructionType::Remove { label } => label,
            InstructionType::Replace {
                label,
                focal_length,
            } => label,
        }
    }
}

struct Instruction {
    box_number: u8,
    instruction_type: InstructionType,
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        let chars = s.chars().collect::<Vec<_>>();
        let last_char_i = chars.len() - 1;
        let instruction_type = if chars[last_char_i] == '-' {
            let label = chars[0..last_char_i].iter().collect::<String>();
            InstructionType::Remove { label }
        } else {
            let splits = s.split('=').collect::<Vec<_>>();
            let focal_length = splits[1].parse::<u8>().unwrap();
            let label = splits[0].to_owned();
            InstructionType::Replace {
                label,
                focal_length,
            }
        };
        Instruction {
            box_number: apply_hash_algorithm(instruction_type.label()),
            instruction_type,
        }
    }

    fn execute(&self, lens_box: &mut Vec<Option<Lens>>) -> () {
        let maybe_lens_index_in_box = lens_box.iter().cloned().position(|maybe_lens| {
            maybe_lens.is_some() && maybe_lens.unwrap().label == self.instruction_type.label()
        });
        match &self.instruction_type {
            InstructionType::Remove { label } => {
                if maybe_lens_index_in_box.is_some() {
                    lens_box[maybe_lens_index_in_box.unwrap()] = None;
                }
            }
            InstructionType::Replace {
                label,
                focal_length,
            } => {
                let new_lens = Lens {
                    label: label.clone(),
                    focal_length: focal_length.clone(),
                };
                if maybe_lens_index_in_box.is_some() {
                    lens_box[maybe_lens_index_in_box.unwrap()] = Some(new_lens);
                } else {
                    lens_box.push(Some(new_lens));
                }
            }
        }
    }
}

fn main() {
    let mut file_content = fs::read_to_string("resources/input_1").unwrap();
    // strip trailing newline
    file_content.truncate(file_content.len() - 1);
    let total = file_content
        .split(",")
        .map(apply_hash_algorithm)
        .map(|n| n as u64)
        .sum::<u64>();
    println!("Part 1 solution: {total}");

    let mut lens_boxes: Vec<Vec<Option<Lens>>> = vec![vec![]; 256];
    for str in file_content.split(",") {
        let instruction = Instruction::new(str);
        let lens_box = &mut lens_boxes[instruction.box_number as usize];
        instruction.execute(lens_box);
    }
    // use filter_map to strip all Nones out of lens boxes
    let total_focusing_power = lens_boxes
        .into_iter()
        .enumerate()
        .map(|(box_i, lens_box)| {
            lens_box
                .iter()
                .filter(|maybe_l| maybe_l.is_some())
                .cloned()
                .enumerate()
                .map(|(lens_i, l)| l.unwrap().focal_length as usize * (lens_i + 1) * (box_i + 1))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("Part 2 solution: {}", total_focusing_power);
}
