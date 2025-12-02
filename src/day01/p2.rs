pub fn solve(input: &str) -> i32 {
    let inputs = super::parse(input);

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p2_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn p2_tests() {
        let input = "R1000";
        let result = solve(input);
        assert_eq!(result, 10);

        let input = "L1000";
        let result = solve(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn p2_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(result, 5961);
    }
}
