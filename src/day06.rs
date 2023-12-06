struct Event {
    time: u64,
    record: u64,
}

impl Event {
    fn distance(&self, press: u64) -> u64 {
        press * (self.time - press)
    }

    fn distances(&self) -> Vec<u64> {
        (0..=self.time).map(|press| self.distance(press)).collect()
    }
}

fn load_events(input: &str) -> Vec<Event> {
    let (times_str, records_str) = input.split_once('\n').unwrap();
    let times = times_str.split_whitespace().skip(1).map(|time| time.parse::<u64>().unwrap());
    let records = records_str.split_whitespace().skip(1).map(|record| record.parse::<u64>().unwrap());
    times.zip(records).map(|(time, record)| Event { time, record }).collect()
}

fn load_event(input: &str) -> Event {
    let (time_str, record_str) = input.split_once('\n').unwrap();
    let time = time_str.replace(' ', "").split_once(':').unwrap().1.parse::<u64>().unwrap();
    let record = record_str.replace(' ', "").split_once(':').unwrap().1.parse::<u64>().unwrap();
    Event { time, record }
}

pub fn number_of_winning_combos_part_1(input: &str) -> u64 {
    load_events(input).iter().map(|event| event.distances().into_iter().filter(|&d| d > event.record).count() as u64).product()
}

pub fn number_of_winning_combos_part_2(input: &str) -> u64 {
    let event = load_event(input);
    let first_winning = (0..=event.time).find(|&press| event.distance(press) > event.record).unwrap();
    let last_winning = (0..=event.time).rfind(|&press| event.distance(press) > event.record).unwrap();
    last_winning - first_winning + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn example_1() {
        assert_eq!(number_of_winning_combos_part_1(EXAMPLE), 288);
    }

    #[test]
    fn example_2() {
        assert_eq!(number_of_winning_combos_part_2(EXAMPLE), 71503);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", number_of_winning_combos_part_1(include_str!("../res/day06.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", number_of_winning_combos_part_2(include_str!("../res/day06.txt")));
    }
}
