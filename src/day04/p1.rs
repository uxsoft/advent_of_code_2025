use std::collections::HashMap;

pub fn score(grid: &super::Grid, x: usize, y: usize) -> usize {
    grid.adjacent_coords8(x, y)
        .iter()
        .filter_map(|(cx, cy)| grid.get(*cx, *cy))
        .map(|c| match c {
            &super::Cell::Empty => 0,
            &super::Cell::Paper => 1,
        })
        .sum()
}

pub fn solve(input: &str) -> usize {
    let grid = super::Grid::parse(input);

    let candidates: HashMap<(usize, usize), char> = grid
        .iter_coords()
        .map(|(x, y)| ((x, y), score(&grid, x, y)))
        .filter(|((x, y), score)| {
            grid.get(*x, *y).unwrap_or(&super::Cell::Empty).is_paper() && *score < 4
        })
        .map(|(pos, _score)| (pos, 'X'))
        .collect();

    println!("{}", grid.highlight(&candidates));

    candidates.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(13, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(1587, result);
    }
}
