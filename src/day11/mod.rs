use std::collections::HashMap;

pub mod p1;
pub mod p2;

#[derive(Debug)]
pub struct Graph<'a> {
    adj: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut adj = HashMap::new();

        for line in input.lines() {
            let (node, edge_str) = line.split_once(": ").unwrap();

            adj.insert(node, edge_str.split_whitespace().collect());
        }

        Self { adj }
    }
}
