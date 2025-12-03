use std::usize;

pub fn max_joltage(slice: &[u8], n: usize) -> usize {
    // Highest joltage cell excluding the last position, it needs to be a two digit number
    let (first_index, first) = slice[..=slice.len() - n]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_index, i)| **i)
        .unwrap();

    // println!("From {slice:?} picked {first}");

    if n > 1 {
        let second = max_joltage(&slice[first_index + 1..], n - 1);
        10_usize.pow(n as u32 - 1) * (*first as usize) + (second)
    } else {
        *first as usize
    }
}

pub fn solve(input: &str) -> usize {
    let batteries = super::parse(input);

    let result = batteries
        .iter()
        //.inspect(|s| println!("Input: {s:?}"))
        .map(|b| max_joltage(b.cells.as_slice(), 12))
        //.inspect(|i| println!("Complete: {i}"))
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(3121910778619, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(173229689350551, result);
    }
}
