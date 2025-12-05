use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn len(range: &RangeInclusive<usize>) -> usize {
    range.end() - range.start() + 1
}

pub fn reduce(ranges: &Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let ranges: Vec<_> = ranges.into_iter().sorted_by_key(|r| r.start()).collect();
    let mut reduced_ranges = vec![];

    let current = ranges.first().unwrap();
    let mut current_start = current.start();
    let mut current_end = current.end();

    for range in ranges.into_iter().skip(1) {
        if range.start() <= current_end {
            current_end = current_end.max(range.end());
        } else {
            reduced_ranges.push(*current_start..=*current_end);
            current_start = range.start();
            current_end = range.end();
        }
    }

    reduced_ranges.push(*current_start..=*current_end);

    reduced_ranges
}

pub fn solve(input: &str) -> usize {
    let (ranges, _ids) = super::parse(input);

    let ranges = reduce(&ranges);

    let range_total: usize = ranges.iter().map(|r| len(r)).sum();

    let range_overlaps = 0;

    range_total - (range_overlaps / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        assert_eq!(len(&(1_usize..=3_usize)), 3);
    }

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(14, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert!(result > 337156057132253);
        assert_eq!(354226555270043, result);
    }
}
