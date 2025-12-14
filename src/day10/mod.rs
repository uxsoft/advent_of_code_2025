pub mod p1;
pub mod p2;
pub mod p2_lp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    s: [bool; 12],
}

impl State {
    pub fn initial() -> Self {
        Self { s: [false; 12] }
    }

    pub fn parse(input: &str) -> Self {
        let mut s = [false; 12];

        for (i, c) in input
            .trim_start_matches("[")
            .trim_end_matches("]")
            .char_indices()
        {
            s[i] = match c {
                '.' => false,
                '#' => true,
                _ => panic!("Error parsing state, encountered an invalid character {c}"),
            };
        }

        Self { s }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Joltage {
    s: [u8; 12],
}

impl Joltage {
    pub fn initial() -> Self {
        Self { s: [0; 12] }
    }

    pub fn parse(input: &str) -> Self {
        let mut s = [0; 12];

        for (i, c) in input
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
            .enumerate()
        {
            s[i] = c.parse().unwrap();
        }

        Self { s }
    }
}

#[derive(Debug)]
pub struct Button {
    id: usize,
    connections: Vec<usize>,
}

impl Button {
    pub fn parse(input: &str, id: usize) -> Self {
        let connections = input
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        Self { id, connections }
    }
}

#[derive(Debug)]
pub struct Problem {
    final_state: State,
    buttons: Vec<Button>,
    joltage_req: Joltage,
}

pub fn parse(input: &str) -> Vec<Problem> {
    input
        .lines()
        .map(|line| {
            let mut final_state = State::initial();
            let mut buttons = vec![];
            let mut joltage_req = Joltage::initial();

            for item in line.split(" ") {
                match item.chars().next().unwrap() {
                    '[' => final_state = State::parse(item),
                    '(' => buttons.push(Button::parse(item, buttons.len())),
                    '{' => joltage_req = Joltage::parse(item),
                    _ => {}
                }
            }

            Problem {
                final_state,
                buttons,
                joltage_req,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = include_str!("example.txt");
        let result = parse(input);
        println!("{:?}", result);
        assert_eq!(24, result.len());
    }
}
