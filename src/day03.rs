use std::mem;

const GEAR: u8 = b'*';

struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position {
            x: x as i64,
            y: y as i64,
        }
    }

    fn near(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

struct Part {
    number: u64,
    positions: Vec<Position>,
}

impl Part {
    fn new() -> Self {
        Part {
            number: 0,
            positions: vec![],
        }
    }
}

struct Symbol {
    kind: u8,
    position: Position,
}

impl Symbol {
    fn new(kind: u8, x: usize, y: usize) -> Self {
        Symbol {
            kind,
            position: Position::new(x, y),
        }
    }
}

fn near_symbol(data: &Vec<&[u8]>, x: usize, y: usize) -> bool {
    let min_x = if x == 0 { 0 } else { x - 1 };
    let min_y = if y == 0 { 0 } else { y - 1 };
    let max_x = if x == data[0].len() - 1 { x } else { x + 1 };
    let max_y = if y == data.len() - 1 { y } else { y + 1 };
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            match data[i][j] {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'.' => {}
                _ => return true,
            }
        }
    }
    false
}

fn load_parts_near_symbols(input: &str) -> Vec<u64> {
    let data: Vec<&[u8]> = input.split('\n').map(|l| l.as_bytes()).collect();
    let y_len = data.len();
    let x_len = data[0].len();
    let mut parts = vec![];
    let mut part = 0u64;
    let mut symbol = false;
    for y in 0..y_len {
        for x in 0..x_len {
            match data[y][x] {
                b'0'..=b'9' => {
                    part = part * 10 + (data[y][x] - b'0') as u64;
                    if !symbol {
                        symbol = near_symbol(&data, x, y);
                    }
                }
                _ => {
                    if symbol && part > 0 {
                        parts.push(part);
                    }
                    part = 0;
                    symbol = false;
                }
            }
        }
        if symbol && part > 0 {
            parts.push(part);
        }
        part = 0;
        symbol = false;
    }
    parts
}

fn load_things(input: &str) -> (Vec<Part>, Vec<Symbol>) {
    let data: Vec<&[u8]> = input.split('\n').map(|l| l.as_bytes()).collect();
    let mut parts = vec![];
    let mut symbols = vec![];
    let mut part = Part::new();
    let y_len = data.len();
    let x_len = data[0].len();
    for y in 0..y_len {
        for x in 0..x_len {
            match data[y][x] {
                b'0'..=b'9' => {
                    part.number = part.number * 10 + (data[y][x] - b'0') as u64;
                    part.positions.push(Position::new(x, y));
                }
                _ => {
                    if data[y][x] != b'.' {
                        symbols.push(Symbol::new(data[y][x], x, y));
                    }
                    if part.number > 0 {
                        parts.push(mem::replace(&mut part, Part::new()));
                    }
                }
            }
        }
        if part.number > 0 {
            parts.push(mem::replace(&mut part, Part::new()));
        }
    }
    (parts, symbols)
}

pub fn sum_of_parts_near_symbols(input: &str) -> u64 {
    load_parts_near_symbols(input).iter().sum::<u64>()
}

pub fn sum_of_gear_ratios(input: &str) -> u64 {
    let (parts, symbols) = load_things(input);
    symbols.iter()
        .filter(|symbol| symbol.kind == GEAR)
        .map(|symbol| parts.iter()
            .filter(|part| part.positions.iter().any(|position| position.near(&symbol.position)))
            .map(|part| part.number)
            .collect::<Vec<u64>>())
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().product::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};

    use crate::read_file_to_string;

    use super::*;

    const EXAMPLE: &str = "467..114..\n\
                       ...*......\n\
                       ..35..633.\n\
                       ......#...\n\
                       617*......\n\
                       .....+.58.\n\
                       ..592.....\n\
                       ......755.\n\
                       ...$.*....\n\
                       .664.598..";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(sum_of_parts_near_symbols(EXAMPLE), 4361);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(sum_of_gear_ratios(EXAMPLE), 467835);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", sum_of_parts_near_symbols(&read_file_to_string("res/day03.txt").with_context(|| "loading day data")?));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", sum_of_gear_ratios(&read_file_to_string("res/day03.txt").with_context(|| "loading day data")?));
        Ok(())
    }
}
