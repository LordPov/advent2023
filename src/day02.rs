use anyhow::{anyhow, Context, Result};

const MAX_RED: u64 = 12;
const MAX_GREEN: u64 = 13;
const MAX_BLUE: u64 = 14;

struct Hand {
    red: u64,
    green: u64,
    blue: u64,
}

impl Hand {
    fn new() -> Self {
        Hand {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn plausible(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u64,
    hands: Vec<Hand>,
}

impl Game {
    fn plausible(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
        self.hands.iter().all(|hand| hand.plausible(max_red, max_green, max_blue))
    }

    fn required_colours(&self) -> Hand {
        let mut maxs = Hand::new();
        for hand in &self.hands {
            maxs.red = maxs.red.max(hand.red);
            maxs.green = maxs.green.max(hand.green);
            maxs.blue = maxs.blue.max(hand.blue);
        }
        maxs
    }
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.split_once(": ") {
            None => Err(anyhow!("game line missing ':' ({})", value)),
            Some((game_str, hands_str)) => {
                let id = game_str[5..].parse::<u64>().with_context(|| format!("Couldn't parse game number ({})", game_str))?;
                let mut hands = vec![];
                for hand_str in hands_str.split("; ") {
                    let mut hand = Hand::new();
                    for colour_str in hand_str.split(", ") {
                        match colour_str.split_once(" ") {
                            None => Err(anyhow!("colour missing ' ' ({})", colour_str))?,
                            Some((num, colour)) => {
                                let num = num.parse::<u64>().with_context(|| format!("Couldn't parse colour count ({})", colour_str))?;
                                match colour {
                                    "red" => hand.red = num,
                                    "green" => hand.green = num,
                                    "blue" => hand.blue = num,
                                    _ => Err(anyhow!("Couldn't parse colour name ({})", colour_str))?,
                                }
                            }
                        }
                    }
                    hands.push(hand);
                }
                Ok(Game { id, hands })
            }
        }
    }
}

fn load_games(input: &str) -> Result<Vec<Game>> {
    input.lines().map(|line| Game::try_from(line)).collect()
}

pub fn sum_of_plausible_games(input: &str) -> Result<u64> {
    Ok(load_games(input)?.iter().filter(|game| game.plausible(MAX_RED, MAX_GREEN, MAX_BLUE)).map(|g| g.id).sum::<u64>())
}

pub fn sum_of_required_colours_powers(input: &str) -> Result<u64> {
    Ok(load_games(input)?.iter().map(|g| g.required_colours().power()).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(sum_of_plausible_games(EXAMPLE)?, 8);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(sum_of_required_colours_powers(EXAMPLE)?, 2286);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", sum_of_plausible_games(include_str!("../res/day02.txt"))?);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", sum_of_required_colours_powers(include_str!("../res/day02.txt"))?);
        Ok(())
    }
}
