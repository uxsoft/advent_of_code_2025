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
    // TODO Optimize to be only checking and removing positions of paper, instead of going through all positions including empty places every time
    let mut grid = super::Grid::parse(input);
    let mut counter = 0;
    loop {
        let candidates: HashMap<(usize, usize), char> = grid
            .iter_coords()
            .map(|(x, y)| ((x, y), score(&grid, x, y)))
            .filter(|((x, y), score)| {
                grid.get(*x, *y).unwrap_or(&super::Cell::Empty).is_paper() && *score < 4
            })
            .map(|(pos, _score)| (pos, 'X'))
            .collect();

        if candidates.is_empty() {
            return counter;
        } else {
            if cfg!(debug_assertions) {
                println!("Removed: {:?}", candidates.len());
            }
            counter += candidates.len();
        }

        for ((x, y), _) in candidates {
            grid.set(x, y, super::Cell::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(43, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(8946, result);
    }
}
