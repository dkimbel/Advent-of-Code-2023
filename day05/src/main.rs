use std::fs;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct AlmanacMap {
    source_start: i64,
    source_end: i64,
    dest_source_diff: i64,
}

impl AlmanacMap {
    fn traverse(&self, source: i64) -> Option<i64> {
        if source >= self.source_start && source <= self.source_end {
            Some(source + self.dest_source_diff)
        } else {
            None
        }
    }

    fn traverse_opposite_direction(&self, dest: i64) -> Option<i64> {
        let dest_start = self.source_start + self.dest_source_diff;
        let dest_end = self.source_end + self.dest_source_diff;
        if dest >= dest_start && dest <= dest_end {
            Some(dest - self.dest_source_diff)
        } else {
            None
        }
    }

    fn traverse_list(maps: &[Self], source: i64) -> i64 {
        for map in maps {
            match map.traverse(source) {
                Some(dest) => {
                    return dest;
                }
                None => (),
            }
        }
        // default behavior: source maps directly to dest
        return source;
    }

    fn traverse_list_opposite_direction(maps: &[Self], dest: i64) -> i64 {
        for map in maps {
            match map.traverse_opposite_direction(dest) {
                Some(source) => {
                    return source;
                }
                None => (),
            }
        }
        // default behavior: dest maps directly to source
        return dest;
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    seed_ranges: Vec<Range>,
    seed_to_soil: Vec<AlmanacMap>,
    soil_to_fertilizer: Vec<AlmanacMap>,
    fertilizer_to_water: Vec<AlmanacMap>,
    water_to_light: Vec<AlmanacMap>,
    light_to_temperature: Vec<AlmanacMap>,
    temperature_to_humidity: Vec<AlmanacMap>,
    humidity_to_location: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_location(&self, seed: i64) -> i64 {
        let soil = AlmanacMap::traverse_list(&self.seed_to_soil, seed);
        let fertilizer = AlmanacMap::traverse_list(&self.soil_to_fertilizer, soil);
        let water = AlmanacMap::traverse_list(&self.fertilizer_to_water, fertilizer);
        let light = AlmanacMap::traverse_list(&self.water_to_light, water);
        let temperature = AlmanacMap::traverse_list(&self.light_to_temperature, light);
        let humidity = AlmanacMap::traverse_list(&self.temperature_to_humidity, temperature);
        AlmanacMap::traverse_list(&self.humidity_to_location, humidity)
    }

    fn location_seed(&self, location: i64) -> i64 {
        let humidity =
            AlmanacMap::traverse_list_opposite_direction(&self.humidity_to_location, location);
        let temperature =
            AlmanacMap::traverse_list_opposite_direction(&self.temperature_to_humidity, humidity);
        let light =
            AlmanacMap::traverse_list_opposite_direction(&self.light_to_temperature, temperature);
        let water = AlmanacMap::traverse_list_opposite_direction(&self.water_to_light, light);
        let fertilizer =
            AlmanacMap::traverse_list_opposite_direction(&self.fertilizer_to_water, water);
        let soil =
            AlmanacMap::traverse_list_opposite_direction(&self.soil_to_fertilizer, fertilizer);
        AlmanacMap::traverse_list_opposite_direction(&self.seed_to_soil, soil)
    }

    fn seed_locations(&self) -> Vec<i64> {
        self.seeds
            .iter()
            .map(|seed| self.seed_location(*seed))
            .collect::<Vec<_>>()
    }

    fn contains_seed(&self, seed: i64) -> bool {
        self.seed_ranges
            .iter()
            .any(|seed_range| seed >= seed_range.start && seed <= seed_range.end)
    }

    fn work_backwards_lowest_location_to_seed(&self) -> i64 {
        let location_ranges = self
            .humidity_to_location
            .iter()
            .map(|map| {
                let location_start = map.source_start + map.dest_source_diff;
                let location_end = map.source_end + map.dest_source_diff;
                Range {
                    start: location_start,
                    end: location_end,
                }
            })
            .collect::<Vec<_>>();

        // TODO: merge overlapping/adjacent ranges if necessary
        let mut lowest_starting_range = location_ranges[0];
        for location_range in location_ranges[1..].iter() {
            if location_range.start < lowest_starting_range.start {
                lowest_starting_range = *location_range;
            }
        }

        let mut curr_location = lowest_starting_range.start;
        while curr_location <= lowest_starting_range.end {
            let seed = self.location_seed(curr_location);
            if self.contains_seed(seed) {
                return curr_location;
            }
            curr_location += 1;
        }
        panic!("Could not find a seed within the lowest-starting location range");
    }

    // fn lowest_location_using_seed_range(&self) -> i64 {
    //     let mut lowest_location = i64::MAX;

    //     for seed_range in self.seed_ranges.iter() {
    //         let mut curr_seed = seed_range.start;
    //         while curr_seed <= seed_range.end {
    //             let location = self.seed_location(curr_seed);
    //             lowest_location = std::cmp::min(lowest_location, location);
    //             curr_seed += 1;
    //         }
    //     }

    //     lowest_location
    // }
}

fn lines_to_map(lines: &[&str]) -> Vec<AlmanacMap> {
    let mut maps = Vec::new();
    for line in lines.iter() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let range = split[2].parse::<i64>().unwrap();
        let source_start = split[1].parse::<i64>().unwrap();
        let source_end = source_start + range - 1;
        let dest_start = split[0].parse::<i64>().unwrap();
        let dest_source_diff = dest_start - source_start;
        maps.push(AlmanacMap {
            source_start,
            source_end,
            dest_source_diff,
        })
    }
    maps
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

    let mut seed_ranges = Vec::new();
    for seed_pair in seeds.chunks(2) {
        let start = seed_pair[0];
        let range = seed_pair[1];
        seed_ranges.push(Range {
            start,
            end: start + range - 1,
        })
    }

    let seed_to_soil = lines_to_map(&split[1].split("\n").collect::<Vec<_>>()[1..]);
    let soil_to_fertilizer = lines_to_map(&split[2].split("\n").collect::<Vec<_>>()[1..]);
    let fertilizer_to_water = lines_to_map(&split[3].split("\n").collect::<Vec<_>>()[1..]);
    let water_to_light = lines_to_map(&split[4].split("\n").collect::<Vec<_>>()[1..]);
    let light_to_temperature = lines_to_map(&split[5].split("\n").collect::<Vec<_>>()[1..]);
    let temperature_to_humidity = lines_to_map(&split[6].split("\n").collect::<Vec<_>>()[1..]);
    let humidity_to_location = lines_to_map(
        &split[7]
            .split("\n")
            .filter(|s| s != &"")
            .collect::<Vec<_>>()[1..],
    );

    let almanac = Almanac {
        seeds,
        seed_ranges,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    // let locations = almanac.seed_locations();
    // let part_1_solution = locations.iter().min().unwrap();
    // println!("Part 1 solution: {part_1_solution}");

    let part_2_solution = almanac.work_backwards_lowest_location_to_seed();
    println!("Part 2 solution: {part_2_solution}");
}
