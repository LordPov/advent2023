const START: usize = 0;  //AAA
const GOAL: usize = 17575;  //ZZZ
const TOTAL: usize = GOAL + 1;


struct Map {
    directions: Vec<usize>,
    paths: [[usize; 2]; TOTAL],
}

impl Map {
    fn steps_to_goal(&self) -> usize {
        self.steps_to_goal_from_location(START)
    }

    fn steps_to_goal_from_location(&self, mut location: usize) -> usize {
        for (i, &direction) in self.directions.iter().enumerate() {
            location = self.paths[location][direction];
            if location == GOAL {
                return i + 1;
            }
        }
        self.directions.len() + self.steps_to_goal_from_location(location)
    }

    fn concurrent_steps_to_goal(&self) -> usize {
        struct Ghost {
            location: usize,
            first_at_z: usize,
            loop_length: usize,
        }

        let mut ghosts: Vec<Ghost> = (0..TOTAL).filter(|&i| self.paths[i][0] != TOTAL).filter(|i| i % 26 == 0).map(|i| Ghost {
            location: i,
            first_at_z: 0,
            loop_length: 0,
        }).collect();
        for ghost in ghosts.iter_mut() {
            let mut steps = 0usize;
            while ghost.loop_length == 0 {
                for &direction in &self.directions {
                    steps += 1;
                    ghost.location = self.paths[ghost.location][direction];
                    if ghost.location % 26 == 25 {
                        if ghost.first_at_z == 0 {
                            ghost.first_at_z = steps;
                        } else {
                            ghost.loop_length = steps - ghost.first_at_z;
                            break;
                        }
                    }
                }
            }
        }

        ghosts.iter().map(|g| g.loop_length).reduce(|a, b| lcm(a, b)).unwrap()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let directions = lines.next().unwrap().bytes().map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            other => panic!("Unexpected direction: {}", other),
        }).collect();

        let mut paths = [[TOTAL; 2]; TOTAL];
        lines.filter(|line| !line.is_empty()).for_each(|line| {
            let location = location_str_to_base_26(&line[0..3]);
            let left = location_str_to_base_26(&line[7..10]);
            let right = location_str_to_base_26(&line[12..15]);
            paths[location as usize] = [left as usize, right as usize];
        });

        Map { directions, paths }
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        a
    } else if b > a {
        gcd(b, a)
    } else {
        while b > 0 {
            let temp = a;
            a = b;
            b = temp % b;
        }
        a
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}
pub fn steps_to_goal(input: &str) -> usize {
    Map::from(input).steps_to_goal()
}

pub fn concurrent_steps_to_goal(input: &str) -> usize {
    Map::from(input).concurrent_steps_to_goal()
}

fn location_str_to_base_26(location: &str) -> u16 {
    location.bytes().map(|b| (b - b'A') as u16).fold(0u16, |result, val| result * 26 + val)
}

#[allow(dead_code)]
fn base_26_to_location_str(location: u16) -> String {
    let mut result = String::new();
    let third = location % 26;
    let second = (location / 26) % 26;
    let first = location / 26 / 26;
    result.push((first as u8 + b'A') as char);
    result.push((second as u8 + b'A') as char);
    result.push((third as u8 + b'A') as char);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_B: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_C: &str = "LR\n\nAAA = (AAB, XXX)\nAAB = (XXX, AAZ)\nAAZ = (AAB, XXX)\nBBA = (BBB, XXX)\nBBB = (BBC, BBC)\nBBC = (BBZ, BBZ)\nBBZ = (BBB, BBB)\nXXX = (XXX, XXX)";

    #[test]
    fn str_to_u16_test() {
        assert_eq!(location_str_to_base_26("AAA") as usize, START);
        assert_eq!(location_str_to_base_26("ZZZ") as usize, GOAL);
    }

    #[test]
    fn example_1() {
        assert_eq!(steps_to_goal(EXAMPLE_A), 2);
        assert_eq!(steps_to_goal(EXAMPLE_B), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(concurrent_steps_to_goal(EXAMPLE_C), 6);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", steps_to_goal(include_str!("../res/day08.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", concurrent_steps_to_goal(include_str!("../res/day08.txt")));
    }
}
