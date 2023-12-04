struct Card {
    winners: Vec<u64>,
    numbers: Vec<u64>,
    count: u64,
}

impl Card {
    fn new(winners: Vec<u64>, numbers: Vec<u64>) -> Self {
        Self { winners, numbers, count: 1 }
    }

    fn winning_count(&self) -> u32 {
        self.numbers.iter().filter(|n| self.winners.contains(n)).count() as u32
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, rest) = value.split_once(':').unwrap();
        let (winners_str, numbers_str) = rest.split_once('|').unwrap();
        Self::new(winners_str.split_whitespace().map(|w| w.parse::<u64>().unwrap()).collect(),
                  numbers_str.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect())
    }
}

fn load_cards(input: &str) -> Vec<Card> {
    input.split('\n').map(Card::from).collect()
}

pub fn sum_of_all_cards_points(input: &str) -> u64 {
    load_cards(input).iter()
        .map(Card::winning_count)
        .map(|w| if w == 0 { 0 } else { 2u64.pow(w - 1) })
        .sum()
}

pub fn total_cards(input: &str) -> u64 {
    let mut cards = load_cards(input);
    for i in 0..cards.len() {
        let winners = cards[i].winning_count() as usize;
        for j in 1..=winners {
            cards[i + j].count += cards[i].count;
        }
    }
    cards.iter().map(|c| c.count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                           Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                           Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                           Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                           Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                           Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example_1() {
        assert_eq!(sum_of_all_cards_points(EXAMPLE), 13);
    }

    #[test]
    fn example_2() {
        assert_eq!(total_cards(EXAMPLE), 30);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", sum_of_all_cards_points(include_str!("../res/day04.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", total_cards(include_str!("../res/day04.txt")));
    }
}
