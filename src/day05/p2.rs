use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let (ranges, _ids) = super::parse(input);

    let result: HashSet<usize> = ranges.into_iter().flat_map(|r| r.into_iter()).collect();

    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(14, result);
    }

    // #[test]
    // fn test_input() {
    //     let input = include_str!("input.txt");
    //     let result = solve(input);
    //     assert_eq!(0, result);
    // }
}
