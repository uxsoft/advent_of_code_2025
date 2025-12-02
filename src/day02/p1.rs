pub fn is_invalid(i: usize) -> bool {
    let str = i.to_string();

    if str.len() % 2 > 0 {
        return false;
    }

    let half = str.len() / 2;

    str[..half] == str[half..]
}

pub fn solve(input: &str) -> usize {
    let ranges = super::parse(input);

    let invalid_ids: Vec<usize> = ranges
        .clone()
        .into_iter()
        .flat_map(|r| r.into_iter().filter(|i| is_invalid(*i)))
        .collect();

    let answer = invalid_ids.iter().sum();

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid() {
        for i in [11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859] {
            assert_eq!(is_invalid(i), true);
        }
        assert_eq!(is_invalid(1234567), false);
    }

    #[test]
    fn p1_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn p1_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(result, 23039913998);
    }
}
