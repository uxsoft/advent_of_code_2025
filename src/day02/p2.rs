pub fn is_invalid(i: usize) -> bool {
    let str = i.to_string();
    let half = str.len() / 2;

    for l in 1..=half {
        let seq = &str[..l];

        // Performance optimization
        if str.len() % l > 0 {
            continue;
        }

        if str.replace(seq, "").len() == 0 {
            return true;
        }
    }

    return false;
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
        for i in [
            11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
            2121212121,
        ] {
            assert_eq!(is_invalid(i), true);
        }
        assert_eq!(is_invalid(1234567), false);
    }

    #[test]
    fn p2_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn p2_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(result, 35950619148);
    }
}
