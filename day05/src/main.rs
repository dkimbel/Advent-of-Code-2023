use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum EntryType {
    Finished, // indicates we're finished searching
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
    entry_type: EntryType,
}

#[derive(Debug)]
struct SearchRange {
    range: Range,
    diff_from_originating_location_range: i64,
}

#[derive(Debug, Copy, Clone)]
struct RangeMap {
    source_start: i64,
    source_end: i64,
    dest_source_diff: i64,
}

impl RangeMap {
    fn from_line(line: &str) -> RangeMap {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let range = split[2].parse::<i64>().unwrap();
        let source_start = split[0].parse::<i64>().unwrap();
        let source_end = source_start + range - 1;
        let dest_start = split[1].parse::<i64>().unwrap();
        let dest_source_diff = dest_start - source_start;
        RangeMap {
            source_start,
            source_end,
            dest_source_diff,
        }
    }
}

#[derive(Debug)]
struct ReversedAlmanac {
    location_to_humidity: Vec<RangeMap>,
    humidity_to_temperature: Vec<RangeMap>,
    temperature_to_light: Vec<RangeMap>,
    light_to_water: Vec<RangeMap>,
    water_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_soil: Vec<RangeMap>,
    soil_to_seed: Vec<RangeMap>,
    seed_to_finished: Vec<RangeMap>,
}

impl ReversedAlmanac {
    fn depth_first_search_by_range(&self) -> Option<i64> {
        let mut search_stack = self.location_ranges_search_stack();

        while let Some(search_range) = search_stack.pop() {
            if search_range.range.entry_type == EntryType::Finished {
                return Some(
                    search_range.range.start - search_range.diff_from_originating_location_range,
                );
            }
            search_stack.append(&mut self.destination_ranges_desc_order(search_range));
        }
        None
    }

    fn destination_ranges_desc_order(&self, search_range: SearchRange) -> Vec<SearchRange> {
        use EntryType::*;
        let (range_maps, destination_type) = match search_range.range.entry_type {
            Location => (&self.location_to_humidity, Humidity),
            Humidity => (&self.humidity_to_temperature, Temperature),
            Temperature => (&self.temperature_to_light, Light),
            Light => (&self.light_to_water, Water),
            Water => (&self.water_to_fertilizer, Fertilizer),
            Fertilizer => (&self.fertilizer_to_soil, Soil),
            Soil => (&self.soil_to_seed, Seed),
            Seed => (&self.seed_to_finished, Finished),
            Finished => panic!("Cannot look up destination ranges for a Finished entry!"),
        };
        let mut destination_ranges: Vec<SearchRange> = Vec::new();
        // Detect any overlap between each range map and our search range. Remember:
        // range maps are sorted in ascending order.
        for range_map in range_maps {
            if range_map.source_end >= search_range.range.start
                && range_map.source_end <= search_range.range.end
            {
                // range map ends within search range (and may or may not start within it too)
                destination_ranges.push(SearchRange {
                    range: Range {
                        start: std::cmp::max(range_map.source_start, search_range.range.start)
                            + range_map.dest_source_diff,
                        end: range_map.source_end + range_map.dest_source_diff,
                        entry_type: destination_type,
                    },
                    diff_from_originating_location_range: search_range
                        .diff_from_originating_location_range
                        + range_map.dest_source_diff,
                });
            } else if range_map.source_start < search_range.range.start
                && range_map.source_end >= search_range.range.end
            {
                // range map completely encapsulates search range
                destination_ranges.push(SearchRange {
                    range: Range {
                        start: search_range.range.start + range_map.dest_source_diff,
                        end: search_range.range.end + range_map.dest_source_diff,
                        entry_type: destination_type,
                    },
                    diff_from_originating_location_range: search_range
                        .diff_from_originating_location_range
                        + range_map.dest_source_diff,
                });
            } else if range_map.source_start >= search_range.range.start
                // range map starts, but does not end, within search range (meaning
                // it's definitely the last range map we need to deal with)
                && range_map.source_start <= search_range.range.end
            {
                destination_ranges.push(SearchRange {
                    range: Range {
                        start: range_map.source_start + range_map.dest_source_diff,
                        end: search_range.range.end + range_map.dest_source_diff,
                        entry_type: destination_type,
                    },
                    diff_from_originating_location_range: search_range
                        .diff_from_originating_location_range
                        + range_map.dest_source_diff,
                });
                break;
            }
        }
        // We found these destinations left to right, starting from the lowest (best) originating
        // location. Since we're ultimately searching using a vec as a stack, reverse the dest
        // ranges so they'll fit properly in our system.
        destination_ranges.reverse();
        destination_ranges
    }

    fn location_ranges_search_stack(&self) -> Vec<SearchRange> {
        // We're using a Vec as our stack to power depth-first search, but we want to start
        // searching from the lowest-numbered location and our original vec was sorted in
        // ascending order. Reverse to switch it to descending so we can start by popping off
        // the lowest possible value.
        let mut search_ranges = self
            .location_to_humidity
            .iter()
            .map(|loc_to_hum| SearchRange {
                range: Range {
                    start: loc_to_hum.source_start,
                    end: loc_to_hum.source_end,
                    entry_type: EntryType::Location,
                },
                diff_from_originating_location_range: 0,
            })
            .collect::<Vec<_>>();
        search_ranges.reverse();
        search_ranges
    }
}

fn lines_to_reversed_map(lines: &[&str], fill_gaps: bool) -> Vec<RangeMap> {
    // 'filled' as in 'any zero-diff ranges have been added to fill gaps';
    // those gaps could appear between ranges, or between 0 an the first range,
    // or between the last range and the max possible int
    let mut range_maps = lines
        .iter()
        .cloned()
        .map(RangeMap::from_line)
        .collect::<Vec<_>>();
    // sort range maps by starting point ascending
    range_maps.sort_by(|a, b| a.source_start.cmp(&b.source_start));
    // fill in any gaps between ranges with a 'zero-diff' range, that just maps any input
    // number directly to the same number as output
    let mut range_maps_with_intermediate_ranges: Vec<RangeMap> = Vec::new();
    for (i, pair) in range_maps.windows(2).enumerate() {
        let first = pair[0];
        let second = pair[1];
        if fill_gaps && (i == 0 && first.source_start > 0) {
            range_maps_with_intermediate_ranges.push(RangeMap {
                source_start: 0,
                source_end: first.source_start - 1,
                dest_source_diff: 0,
            });
        }
        range_maps_with_intermediate_ranges.push(first);
        let first_end = first.source_end;
        let second_start = second.source_start;
        // the ranges are inclusive on both start and end
        if fill_gaps && (second_start - first_end > 1) {
            range_maps_with_intermediate_ranges.push(RangeMap {
                source_start: first_end + 1,
                source_end: second_start - 1,
                dest_source_diff: 0,
            });
        }
        if i + 2 >= range_maps.len() {
            // we've reached the last pair of values -- we're checking i+2 instead
            // of i+1 because windows() stops at the last full window
            range_maps_with_intermediate_ranges.push(second);
            if fill_gaps && (second.source_end < i64::MAX) {
                range_maps_with_intermediate_ranges.push(RangeMap {
                    source_start: second.source_end + 1,
                    source_end: i64::MAX,
                    dest_source_diff: 0,
                });
            }
        }
    }
    range_maps_with_intermediate_ranges
}

fn part_2_seed_to_finished(seeds: &[i64]) -> Vec<RangeMap> {
    seeds
        .chunks(2)
        .map(|seed_pair| {
            let start = seed_pair[0];
            let range = seed_pair[1];
            RangeMap {
                source_start: start,
                source_end: start + range - 1,
                dest_source_diff: 0,
            }
        })
        .collect::<Vec<_>>()
}

fn part_1_seed_to_finished(seeds: &[i64]) -> Vec<RangeMap> {
    seeds
        .iter()
        .map(|&seed| RangeMap {
            source_start: seed,
            source_end: seed,
            dest_source_diff: 0,
        })
        .collect::<Vec<_>>()
}

fn main() {
    let file_content = fs::read_to_string("resources/input_1").unwrap();
    let split = file_content.split("\n\n").collect::<Vec<_>>();
    let seeds_str = split[0].split(": ").collect::<Vec<_>>()[1];
    let seeds = seeds_str
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let soil_to_seed = lines_to_reversed_map(&split[1].split("\n").collect::<Vec<_>>()[1..], true);
    let fertilizer_to_soil =
        lines_to_reversed_map(&split[2].split("\n").collect::<Vec<_>>()[1..], true);
    let water_to_fertilizer =
        lines_to_reversed_map(&split[3].split("\n").collect::<Vec<_>>()[1..], true);
    let light_to_water =
        lines_to_reversed_map(&split[4].split("\n").collect::<Vec<_>>()[1..], true);
    let temperature_to_light =
        lines_to_reversed_map(&split[5].split("\n").collect::<Vec<_>>()[1..], true);
    let humidity_to_temperature =
        lines_to_reversed_map(&split[6].split("\n").collect::<Vec<_>>()[1..], true);
    let location_to_humidity = lines_to_reversed_map(
        &split[7]
            .split("\n")
            .filter(|s| s != &"")
            .collect::<Vec<_>>()[1..],
        true,
    );

    let part_1_seed_to_finished = part_1_seed_to_finished(&seeds);
    let part_2_seed_to_finished = part_2_seed_to_finished(&seeds);

    let part_1_reversed_almanac: ReversedAlmanac = ReversedAlmanac {
        location_to_humidity: location_to_humidity.clone(),
        humidity_to_temperature: humidity_to_temperature.clone(),
        temperature_to_light: temperature_to_light.clone(),
        light_to_water: light_to_water.clone(),
        water_to_fertilizer: water_to_fertilizer.clone(),
        fertilizer_to_soil: fertilizer_to_soil.clone(),
        soil_to_seed: soil_to_seed.clone(),
        seed_to_finished: part_1_seed_to_finished,
    };
    let part_2_reversed_almanac: ReversedAlmanac = ReversedAlmanac {
        location_to_humidity,
        humidity_to_temperature,
        temperature_to_light,
        light_to_water,
        water_to_fertilizer,
        fertilizer_to_soil,
        soil_to_seed,
        seed_to_finished: part_2_seed_to_finished,
    };

    let part_1_solution = part_1_reversed_almanac
        .depth_first_search_by_range()
        .unwrap();
    println!("Part 1 solution: {part_1_solution}");

    let part_2_solution = part_2_reversed_almanac
        .depth_first_search_by_range()
        .unwrap();
    println!("Part 2 solution: {part_2_solution}");
}
