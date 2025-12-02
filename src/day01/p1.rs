pub fn solve(input: &str) -> i32 {
    let inputs = super::parse(input);

    let (_final_sum, crosses) = inputs.into_iter().fold((50, 0), |(acc, v), i| {
        let new_acc = (acc + i).rem_euclid(100);
        let new_v = if new_acc == 0 { v + 1 } else { v };
        (new_acc, new_v)
    });

    crosses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn p1_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(result, 980);
    }
}
