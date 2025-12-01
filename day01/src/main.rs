use std::ops::Add;

fn parse(input: String) -> Vec<i32> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let minus = if line.starts_with("L") { -1 } else { 1 };
            let value = line.trim_start_matches(&['L', 'R']).parse::<i32>().unwrap();
            minus * value
        })
        .collect()
}

fn solve1(input: String) -> i32 {
    let inputs = parse(input);

    let (_final_sum, crosses) = inputs.into_iter().fold((50, 0), |(acc, v), i| {
        // println!("acc: {}, i: {}", acc, i);
        let new_acc = acc.add(i).rem_euclid(100);
        let new_v = if new_acc == 0 { v + 1 } else { v };
        (new_acc, new_v)
    });

    crosses
}

fn solve2(input: String) -> i32 {
    let inputs = parse(input);

    let mut sum = 50;
    let mut value = 0;
    for i in inputs {
        let new_sum = sum + i;

        value += new_sum.div_euclid(100).abs();

        // we're going from 0, which was already counted
        if i < 0 && sum == 0 {
            value -= 1;
        }

        // 5 --> 0 => 1
        // 5 --> -100 => 2 but div_euclid(-100) is still only -1
        if i < 0 && new_sum.rem_euclid(100) == 0 {
            value += 1;
        }

        sum = new_sum.rem_euclid(100);
    }

    value
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let input = include_str!("../inputs/example.txt");
        let result = solve1(input.to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn p1_input() {
        let input = include_str!("../inputs/input.txt");
        let result = solve1(input.to_string());
        assert_eq!(result, 980);
    }

    #[test]
    fn p2_example() {
        let input = include_str!("../inputs/example.txt");
        let result = solve2(input.to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn p2_tests() {
        let input = "R1000";
        let result = solve2(input.to_string());
        assert_eq!(result, 10);

        let input = "L1000";
        let result = solve2(input.to_string());
        assert_eq!(result, 10);
    }

    #[test]
    fn p2_input() {
        let input = include_str!("../inputs/input.txt");
        let result = solve2(input.to_string());
        assert_eq!(result, 5961);
    }
}
