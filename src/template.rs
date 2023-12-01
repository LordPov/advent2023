use anyhow::{Context, Result};

#[cfg(test)]
mod tests {
    use crate::read_file_to_string;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Part 1: {}", read_file_to_string("res/day01.txt").with_context(|| "loading day data")?.len());
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Part 2: {}", read_file_to_string("res/day01.txt").with_context(|| "loading day data")?.len());
        Ok(())
    }
}
