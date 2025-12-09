use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    let boxes = input
        .lines()
        .map(super::Point3d::parse)
        .collect::<Vec<super::Point3d>>();

    let distances: HashMap<(usize, usize), _> = boxes
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
        //.sorted_by(|a, b| distances[a].partial_cmp(&distances[b]).unwrap())
        // performance optimization
        .sorted_by_cached_key(|i| distances[i])
        .collect::<Vec<_>>();

    let mut uf = super::UnionFind::new();

    for (x, y) in order {
        uf.union(*x, *y);
        if uf.parent.len() == boxes.len() - 1 {
            return boxes[*y].x * boxes[*x].x;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(25272, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(7499461416, result);
    }
}
