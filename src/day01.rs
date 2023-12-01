use anyhow::{Context, Result};

fn calibration_values(input: &str, digits_only: bool) -> Result<Vec<u32>> {
    let function = if digits_only { calibration_value_digits_only } else { calibration_value_words_allowed };
    input.split_whitespace().map(|line| function(line)).collect()
}

fn calibration_value_digits_only(line: &str) -> Result<u32> {
    let first = line.chars().find(char::is_ascii_digit).with_context(|| format!("couldn't find first digit in {}", line))?;
    let last = line.chars().rfind(char::is_ascii_digit).with_context(|| format!("couldn't find last digit in {}", line))?;
    Ok(first.to_digit(10).with_context(|| "parsing first")? * 10 + last.to_digit(10).with_context(|| "parsing last")?)
}

fn calibration_value_words_allowed(line: &str) -> Result<u32> {
    Ok(find_first(line.as_bytes()).with_context(|| "finding first")? * 10 + find_last(line.as_bytes()).with_context(|| "finding last")?)
}

fn find_first(line: &[u8]) -> Option<u32> {
    for i in 0..line.len() {
        if let Some(num) = number_at_pos(line, i) {
            return Some(num);
        }
    }
    None
}

fn find_last(line: &[u8]) -> Option<u32> {
    for i in (0..line.len()).rev() {
        if let Some(num) = number_at_pos(line, i) {
            return Some(num);
        }
    }
    None
}

fn number_at_pos(line: &[u8], pos: usize) -> Option<u32> {
    match line[pos] {
        b'0' => Some(0),
        b'1' => Some(1),
        b'2' => Some(2),
        b'3' => Some(3),
        b'4' => Some(4),
        b'5' => Some(5),
        b'6' => Some(6),
        b'7' => Some(7),
        b'8' => Some(8),
        b'9' => Some(9),
        b'o' => {
            if line[pos..].starts_with(b"one") {
                Some(1)
            } else {
                None
            }
        }
        b't' => {
            if line[pos..].starts_with(b"two") {
                Some(2)
            } else if line[pos..].starts_with(b"three") {
                Some(3)
            } else {
                None
            }
        }
        b'f' => {
            if line[pos..].starts_with(b"four") {
                Some(4)
            } else if line[pos..].starts_with(b"five") {
                Some(5)
            } else {
                None
            }
        }
        b's' => {
            if line[pos..].starts_with(b"six") {
                Some(6)
            } else if line[pos..].starts_with(b"seven") {
                Some(7)
            } else {
                None
            }
        }
        b'e' => {
            if line[pos..].starts_with(b"eight") {
                Some(8)
            } else {
                None
            }
        }
        b'n' => {
            if line[pos..].starts_with(b"nine") {
                Some(9)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::read_file_to_string;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        const EXAMPLE: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        assert_eq!(calibration_values(EXAMPLE, true)?.iter().sum::<u32>(), 142);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        const EXAMPLE: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        assert_eq!(calibration_values(EXAMPLE, false)?.iter().sum::<u32>(), 281);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", calibration_values(&read_file_to_string("res/day01.txt")?, true)?.iter().sum::<u32>());
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", calibration_values(&read_file_to_string("res/day01.txt")?, false)?.iter().sum::<u32>());
        Ok(())
    }
}
