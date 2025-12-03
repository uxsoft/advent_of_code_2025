use std::usize;

use super::Battery;

pub fn max_joltage(battery: &Battery) -> usize {
    // Highest joltage cell excluding the last position, it needs to be a two digit number
    let (first_index, first) = battery
        .cells
        .split_last()
        .unwrap()
        .1
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_index, i)| **i)
        .unwrap();
    let second = battery.cells.iter().skip(first_index + 1).max().unwrap();

    10_usize * (*first as usize) + (*second as usize)
}

pub fn solve(input: &str) -> usize {
    let batteries = super::parse(input);

    let result = batteries.iter().map(max_joltage).sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(357, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert!(result > 17258);
        assert_eq!(17445, result);
    }
}
