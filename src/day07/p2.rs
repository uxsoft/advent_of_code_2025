use std::collections::HashMap;

fn beam(
    pos: (usize, usize),
    grid: &super::Grid,
    mut cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if pos.1 >= grid.dim_y {
        return 1;
    }
    if cache.contains_key(&pos) {
        return *cache.get(&pos).unwrap();
    }

    if grid.prisms.contains(&pos) {
        let left = if pos.0 > 0 {
            beam((pos.0 - 1, pos.1), &grid, &mut cache)
        } else {
            0
        };

        let right = if pos.0 < grid.dim_x - 1 {
            beam((pos.0 + 1, pos.1), &grid, &mut cache)
        } else {
            0
        };

        cache.insert(pos, left + right);
        return left + right;
    } else {
        // Continue down
        return beam((pos.0, pos.1 + 1), &grid, &mut cache);
    }
}

pub fn solve(input: &str) -> usize {
    let grid = super::Grid::parse(input);
    let mut cache = HashMap::new();

    let splits = beam(grid.start, &grid, &mut cache);

    splits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(40, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(18818811755665, result);
    }
}
