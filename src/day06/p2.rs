use itertools::Itertools;

struct StateMachine {
    totals: Vec<usize>,
    numbers: Vec<usize>,
    num_str: String,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            totals: vec![],
            numbers: vec![],
            num_str: String::new(),
        }
    }

    fn push_char(&mut self, c: char) {
        self.num_str.push(c)
    }

    fn push_num(&mut self) {
        if let Ok(num) = self.num_str.parse::<usize>() {
            self.numbers.push(num);
            self.num_str.clear();
        }
    }
    fn push_total_sum(&mut self) {
        self.push_num();
        self.totals.push(self.numbers.iter().sum::<usize>());
        self.numbers.clear();
    }

    fn push_total_product(&mut self) {
        self.push_num();
        self.totals.push(self.numbers.iter().product::<usize>());
        self.numbers.clear();
    }
}

pub fn solve(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let dim_y = lines.len();
    let dim_x = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut sm = StateMachine::new();

    for x in (0..dim_x).rev() {
        for y in 0..dim_y {
            let val = lines[y].get(x).unwrap_or(&' ');
            match val {
                &' ' => (),
                &'+' => {
                    sm.push_total_sum();
                }
                &'*' => {
                    sm.push_total_product();
                }
                n => sm.push_char(*n),
            }
        }
        sm.push_num()
    }

    sm.totals.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(3263827, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(11371597126232, result);
    }
}
