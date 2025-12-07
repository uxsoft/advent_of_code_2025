use std::collections::HashSet;

fn beam(
    pos: (usize, usize),
    grid: &super::Grid,
    mut beam_cache: &mut HashSet<(usize, usize)>,
    mut splitter_cache: &mut HashSet<(usize, usize)>,
) {
    if pos.1 >= grid.dim_y {
        return;
    }

    if grid.prisms.contains(&pos) {
        // Split
        if splitter_cache.contains(&pos) {
            return;
        } else {
            splitter_cache.insert(pos);
        }

        if pos.0 > 0 && !beam_cache.contains(&(pos.0 - 1, pos.1)) {
            beam_cache.insert((pos.0 - 1, pos.1));
            beam(
                (pos.0 - 1, pos.1),
                &grid,
                &mut beam_cache,
                &mut splitter_cache,
            )
        };

        if pos.0 < grid.dim_x - 1 && !beam_cache.contains(&(pos.0 + 1, pos.1)) {
            beam_cache.insert((pos.0 + 1, pos.1));
            beam(
                (pos.0 + 1, pos.1),
                &grid,
                &mut beam_cache,
                &mut splitter_cache,
            )
        };
    } else {
        // Continue down
        return beam(
            (pos.0, pos.1 + 1),
            &grid,
            &mut beam_cache,
            &mut splitter_cache,
        );
    }
}

pub fn solve(input: &str) -> usize {
    let grid = super::Grid::parse(input);
    let mut beam_cache = HashSet::new();
    let mut splitter_cache = HashSet::new();

    let splits = beam(grid.start, &grid, &mut beam_cache, &mut splitter_cache);

    splitter_cache.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(21, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(1, result);
    }
}
