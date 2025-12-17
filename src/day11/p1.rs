use std::collections::HashMap;

use super::Graph;

pub fn count_paths(g: &Graph, cache: &mut HashMap<&str, usize>, from: &str, to: &str) -> usize {
    g.adj
        .get(from)
        .unwrap()
        .iter()
        .map(move |n| match n {
            n if *n == to => 1,
            _ => count_paths(g, cache, n, to),
        })
        .sum()
}

pub fn solve(input: &str) -> usize {
    let graph = super::Graph::parse(input);
    let mut cache = HashMap::new();

    // println!("{graph:?}");

    count_paths(&graph, &mut cache, "you", "out")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(5, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(603, result);
    }
}
