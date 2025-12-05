pub mod p1;
pub mod p2;

pub fn parse(input: &str) -> (Vec<std::ops::RangeInclusive<usize>>, Vec<usize>) {
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|l| {
            let (from_str, to_str) = l.split_once("-").unwrap();
            from_str.parse::<usize>().unwrap()..=to_str.parse().unwrap()
        })
        .collect();

    let ids = ids_str.lines().map(|i| i.parse().unwrap()).collect();

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("example.txt");
        let (ranges, ids) = parse(input);
        println!("{:?}", ranges);
        println!("{:?}", ids);
        assert_eq!(4, ranges.len());
        assert_eq!(6, ids.len());
    }

    // #[test]
    // fn test_input() {
    //     let input = include_str!("input.txt");
    //     let result = solve(input);
    //     assert_eq!(1587, result);
    // }
}
