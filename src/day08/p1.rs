use std::collections::BTreeMap;

use itertools::Itertools;

pub fn solve(input: &str, iterations: usize) -> usize {
    let boxes = input
        .lines()
        .map(super::Point3d::parse)
        .collect::<Vec<super::Point3d>>();

    let distances: BTreeMap<(usize, usize), _> = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            boxes
                .iter()
                .enumerate()
                .filter(move |(j, _)| i > *j)
                .map(move |(j, b)| ((i, j), a.distance(&b)))
        })
        .collect();

    let order = distances
        .keys()
        .sorted_by(|a, b| distances[a].partial_cmp(&distances[b]).unwrap())
        .collect::<Vec<_>>();

    let mut uf = super::UnionFind::new();

    for (x, y) in order.iter().take(iterations) {
        uf.union(*x, *y);
    }

    uf.sizes().iter().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input, 10);
        assert_eq!(40, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input, 1000);
        assert_eq!(29406, result);
    }
}
