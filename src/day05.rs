use std::str::Split;

struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Almanac {
    fn destination_for_source(source: i64, maps: &Vec<Mapping>) -> i64 {
        for map in maps {
            if let Some(destination) = map.map(source) {
                return destination;
            }
        }
        source
    }

    fn location_for_seed(&self, seed: i64) -> i64 {
        let soil = Self::destination_for_source(seed, &self.seed_to_soil);
        let fertilizer = Self::destination_for_source(soil, &self.soil_to_fertilizer);
        let water = Self::destination_for_source(fertilizer, &self.fertilizer_to_water);
        let light = Self::destination_for_source(water, &self.water_to_light);
        let temperature = Self::destination_for_source(light, &self.light_to_temperature);
        let humidity = Self::destination_for_source(temperature, &self.temperature_to_humidity);
        let location = Self::destination_for_source(humidity, &self.humidity_to_location);
        location
    }
}

struct Mapping {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl Mapping {
    fn map(&self, source: i64) -> Option<i64> {
        let source_diff = source - self.source_start;
        if source_diff >= 0 && source_diff < self.range {
            Some(self.destination_start + source_diff)
        } else {
            None
        }
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let mut it = value.splitn(3, ' ');
        Mapping {
            destination_start: it.next().unwrap().parse::<i64>().unwrap(),
            source_start: it.next().unwrap().parse::<i64>().unwrap(),
            range: it.next().unwrap().parse::<i64>().unwrap(),
        }
    }
}

pub fn lowest_location_for_seed(input: &str) -> i64 {
    let almanac = load_almanac(input);
    almanac.seeds.iter().map(|&seed| almanac.location_for_seed(seed)).min().unwrap()
}

fn load_almanac(input: &str) -> Almanac {
    fn load_mappings(lines: &mut Split<&str>) -> Vec<Mapping> {
        let mut mappings = vec![];
        lines.next();  //skip heading
        let mut line = lines.next().unwrap();
        while !line.is_empty() {
            mappings.push(Mapping::from(line));
            line = match lines.next() {
                None => return mappings,
                Some(line) => line,
            };
        }
        mappings
    }

    let mut lines = input.split("\n");

    let seeds = lines.next().unwrap()[7..].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
    lines.next(); //skip newline

    Almanac {
        seeds,
        seed_to_soil: load_mappings(&mut lines),
        soil_to_fertilizer: load_mappings(&mut lines),
        fertilizer_to_water: load_mappings(&mut lines),
        water_to_light: load_mappings(&mut lines),
        light_to_temperature: load_mappings(&mut lines),
        temperature_to_humidity: load_mappings(&mut lines),
        humidity_to_location: load_mappings(&mut lines),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13\n\
                           \n\
                           seed-to-soil map:\n\
                           50 98 2\n\
                           52 50 48\n\
                           \n\
                           soil-to-fertilizer map:\n\
                           0 15 37\n\
                           37 52 2\n\
                           39 0 15\n\
                           \n\
                           fertilizer-to-water map:\n\
                           49 53 8\n\
                           0 11 42\n\
                           42 0 7\n\
                           57 7 4\n\
                           \n\
                           water-to-light map:\n\
                           88 18 7\n\
                           18 25 70\n\
                           \n\
                           light-to-temperature map:\n\
                           45 77 23\n\
                           81 45 19\n\
                           68 64 13\n\
                           \n\
                           temperature-to-humidity map:\n\
                           0 69 1\n\
                           1 0 69\n\
                           \n\
                           humidity-to-location map:\n\
                           60 56 37\n\
                           56 93 4";

    #[test]
    fn example_1() {
        lowest_location_for_seed(EXAMPLE);
    }

    #[test]
    fn example_2() {
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", lowest_location_for_seed(include_str!("../res/day05.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", include_str!("../res/day05.txt").len());
    }
}
