use std::fs;

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
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Vec<AlmanacMap>,
    soil_to_fertilizer: Vec<AlmanacMap>,
    fertilizer_to_water: Vec<AlmanacMap>,
    water_to_light: Vec<AlmanacMap>,
    light_to_temperature: Vec<AlmanacMap>,
    temperature_to_humidity: Vec<AlmanacMap>,
    humidity_to_location: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_locations(&self) -> Vec<i64> {
        self.seeds
            .iter()
            .map(|seed| {
                let soil = AlmanacMap::traverse_list(&self.seed_to_soil, *seed);
                let fertilizer = AlmanacMap::traverse_list(&self.soil_to_fertilizer, soil);
                let water = AlmanacMap::traverse_list(&self.fertilizer_to_water, fertilizer);
                let light = AlmanacMap::traverse_list(&self.water_to_light, water);
                let temperature = AlmanacMap::traverse_list(&self.light_to_temperature, light);
                let humidity =
                    AlmanacMap::traverse_list(&self.temperature_to_humidity, temperature);
                AlmanacMap::traverse_list(&self.humidity_to_location, humidity)
            })
            .collect::<Vec<_>>()
    }
}

fn lines_to_map(lines: &[&str]) -> Vec<AlmanacMap> {
    let mut maps = Vec::new();
    for line in lines.iter() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let range = split[2].parse::<i64>().unwrap();
        let source_start = split[1].parse::<i64>().unwrap();
        let source_end = source_start + range;
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
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    let locations = almanac.seed_locations();
    let part_1_solution = locations.iter().min().unwrap();
    println!("Part 1 solution: {part_1_solution}");
}
