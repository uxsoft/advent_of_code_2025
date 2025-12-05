pub fn solve(input: &str) -> usize {
    let (ranges, ids) = super::parse(input);

    let result = ids
        .iter()
        .filter(|id| ranges.iter().find(|r| r.contains(id)).is_some())
        .count();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(3, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(726, result);
    }
}
