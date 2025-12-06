use itertools::Itertools;

enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn parse(input: &str) -> Self {
        match input {
            "*" => Op::Mul,
            "+" => Op::Add,
            _ => panic!("Invalid operator"),
        }
    }

    pub fn apply(&self, seq: impl Iterator<Item = usize>) -> usize {
        seq.reduce(|acc, x| match self {
            Op::Add => acc + x,
            Op::Mul => acc * x,
        })
        .unwrap_or(0)
    }
}

pub fn solve(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec();
    let dim_x = lines[0].len();
    let dim_y = lines.len();

    let result = (0..dim_x)
        .map(|x| {
            let op = Op::parse(lines[dim_y - 1][x]);
            let values = (0..dim_y - 1).map(|y| lines[y][x].parse().unwrap());
            let result = op.apply(values);
            result
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(4277556, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(5552221122013, result);
    }
}
