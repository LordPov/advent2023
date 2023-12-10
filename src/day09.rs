use std::collections::VecDeque;

pub fn sum_of_extrapolated_future_values(input: &str) -> i64 {
    let mut oasis = load_data(input);
    extrapolate_values(&mut oasis);
    oasis.iter().map(|v| *v.back().unwrap()).sum()
}

pub fn sum_of_extrapolated_past_values(input: &str) -> i64 {
    let mut oasis = load_data(input);
    extrapolate_values(&mut oasis);
    oasis.iter().map(|v| *v.front().unwrap()).sum()
}

fn extrapolate_values(values: &mut Vec<VecDeque<i64>>) {
    values.iter_mut().for_each(|v| extrapolate_value(v));
}

fn extrapolate_value(values: &mut VecDeque<i64>) {
    let (extrapolated_past, extrapolated_future) = if values.iter().all(|&v| v == 0) {
        (0, 0)
    } else {
        let mut differences: VecDeque<i64> = values.iter().zip(values.iter().skip(1)).map(|(&a, &b)| b - a).collect();
        extrapolate_value(&mut differences);
        (values.front().unwrap() - differences.front().unwrap(), values.back().unwrap() + differences.back().unwrap())
    };
    values.push_front(extrapolated_past);
    values.push_back(extrapolated_future);
}

fn load_data(input: &str) -> Vec<VecDeque<i64>> {
    input.lines().map(|l| l.split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect::<VecDeque<i64>>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn example_1() {
        assert_eq!(sum_of_extrapolated_future_values(EXAMPLE), 114);
    }

    #[test]
    fn example_2() {
        assert_eq!(sum_of_extrapolated_past_values(EXAMPLE), 2);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", sum_of_extrapolated_future_values(include_str!("../res/day09.txt")));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", sum_of_extrapolated_past_values(include_str!("../res/day09.txt")));
    }
}
