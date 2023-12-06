struct Event {
    time: u64,
    record: u64,
}

impl Event {
    fn button_presses(&self) -> Vec<RaceResult> {
        (0..=self.time).map(|press| RaceResult::new(press, press * (self.time - press))).collect()
    }
}

struct RaceResult {
    _press: u64,
    distance: u64,
}

impl RaceResult {
    fn new(press: u64, distance: u64) -> Self {
        RaceResult {
            _press: press,
            distance,
        }
    }
}

fn load_events(input: &str) -> Vec<Event> {
    let (times_str, records_str) = input.split_once('\n').unwrap();
    let times = times_str.split_whitespace().skip(1).map(|time| time.parse::<u64>().unwrap());
    let records = records_str.split_whitespace().skip(1).map(|record| record.parse::<u64>().unwrap());
    times.zip(records).map(|(time, record)| Event { time, record }).collect()
}

pub fn number_of_winning_combos(input: &str) -> u64 {
    load_events(input).iter().map(|event| event.button_presses().iter().filter(|r| r.distance > event.record).count() as u64).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn example_1() {
        assert_eq!(number_of_winning_combos(EXAMPLE), 288);
    }

    #[test]
    fn example_2() {
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", number_of_winning_combos(include_str!("../res/day06.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", include_str!("../res/day06.txt").len());
    }
}
