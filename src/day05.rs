use std::str::Split;

const MAX_IDS: i64 = 100_000_000_000;

struct Almanac {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl From<&mut Split<'_, &str>> for Almanac {
    fn from(lines: &mut Split<&str>) -> Self {
        fn load_mappings(lines: &mut Split<&str>) -> Vec<Mapping> {
            let mut sparse_mappings = vec![];
            lines.next();  //skip heading
            let mut line = lines.next().unwrap();
            while !line.is_empty() {
                sparse_mappings.push(Mapping::from(line));
                line = match lines.next() {
                    None => break,
                    Some(line) => line,
                };
            }
            sparse_mappings.sort_unstable_by_key(|m| m.source_start);
            let mut mappings = vec![];
            if sparse_mappings[0].source_start != 0 {
                mappings.push(Mapping {
                    destination_start: 0,
                    source_start: 0,
                    range: sparse_mappings[0].source_start,
                })
            }
            mappings.push(sparse_mappings[0].clone());
            for (a, b) in sparse_mappings.iter().zip(sparse_mappings.iter().skip(1)) {
                let gap_start = a.source_start + a.range;
                if b.source_start != (gap_start) {
                    mappings.push(Mapping {
                        destination_start: gap_start,
                        source_start: gap_start,
                        range: b.source_start - gap_start,
                    })
                }
                mappings.push(b.clone());
            }
            let last = mappings.len() - 1;
            let start = mappings[last].source_start + mappings[last].range;
            mappings.push(Mapping {
                destination_start: start,
                source_start: start,
                range: MAX_IDS - start,
            });
            mappings
        }

        Almanac {
            seed_to_soil: load_mappings(lines),
            soil_to_fertilizer: load_mappings(lines),
            fertilizer_to_water: load_mappings(lines),
            water_to_light: load_mappings(lines),
            light_to_temperature: load_mappings(lines),
            temperature_to_humidity: load_mappings(lines),
            humidity_to_location: load_mappings(lines),
        }
    }
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

    fn destinations_for_source(source_start: i64, source_end: i64, maps: &Vec<Mapping>) -> Vec<(i64, i64)> {
        let mut dests = vec![];
        for m in maps {
            let dest_adj = m.destination_start - m.source_start;
            let dest_start = source_start.max(m.source_start) + dest_adj;
            let dest_end = source_end.min(m.source_end()) + dest_adj;
            if dest_start <= dest_end {
                dests.push((dest_start, dest_end));
            }
        }
        dests
    }

    fn seed_paths(&self, seeds: Vec<(i64, i64)>) -> i64 {
        let mut locations = vec![];
        for (seed_start, seed_end) in seeds.into_iter().map(|(s, r)| (s, s + r - 1)) {
            for (soil_start, soil_end) in Self::destinations_for_source(seed_start, seed_end, &self.seed_to_soil) {
                for (fertilizer_start, fertilizer_end) in Self::destinations_for_source(soil_start, soil_end, &self.soil_to_fertilizer) {
                    for (water_start, water_end) in Self::destinations_for_source(fertilizer_start, fertilizer_end, &self.fertilizer_to_water) {
                        for (light_start, light_end) in Self::destinations_for_source(water_start, water_end, &self.water_to_light) {
                            for (temperature_start, temperature_end) in Self::destinations_for_source(light_start, light_end, &self.light_to_temperature) {
                                for (humidity_start, humidity_end) in Self::destinations_for_source(temperature_start, temperature_end, &self.temperature_to_humidity) {
                                    for (location_start, _location_end) in Self::destinations_for_source(humidity_start, humidity_end, &self.humidity_to_location) {
                                        locations.push(location_start);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        locations.sort_unstable();
        locations[0]
    }
}

#[derive(Clone, Debug)]
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

    fn source_end(&self) -> i64 {
        self.source_start + self.range - 1
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

pub fn lowest_location_for_individual_seeds(input: &str) -> i64 {
    let mut lines = input.split("\n");
    let seeds = lines.next().unwrap()[7..].split_whitespace().map(|s| s.parse::<i64>().unwrap());
    lines.next(); //skip newline
    let almanac = Almanac::from(&mut lines);
    seeds.map(|seed| almanac.location_for_seed(seed)).min().unwrap()
}

pub fn lowest_location_for_seed_ranges(input: &str) -> i64 {
    let mut lines = input.split("\n");
    let seeds: Vec<i64> = lines.next().unwrap()[7..].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
    let seeds: Vec<(i64, i64)> = seeds.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])).collect();
    lines.next(); //skip newline
    Almanac::from(&mut lines).seed_paths(seeds)
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
        assert_eq!(lowest_location_for_individual_seeds(EXAMPLE), 35);
    }

    #[test]
    fn example_2() {
        assert_eq!(lowest_location_for_seed_ranges(EXAMPLE), 46);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", lowest_location_for_individual_seeds(include_str!("../res/day05.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", lowest_location_for_seed_ranges(include_str!("../res/day05.txt")));
    }
}
