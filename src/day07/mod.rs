use std::collections::HashSet;

pub mod p1;
pub mod p2;

#[derive(Debug)]
pub struct Grid {
    start: (usize, usize),
    prisms: HashSet<(usize, usize)>,
    dim_x: usize,
    dim_y: usize,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let mut start = (0, 0);
        let mut prisms = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '^' => {
                        prisms.insert((x, y));
                    }
                    'S' => {
                        start = (x, y);
                    }
                    _ => (),
                }
            }
        }

        let dim_y = input.lines().count();
        let dim_x = input.lines().next().unwrap().len();

        Self {
            start,
            prisms,
            dim_x,
            dim_y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("example.txt");
        let grid = Grid::parse(input);

        println!("{:?}", grid);
        assert_eq!((7, 0), grid.start);
        assert_eq!(22, grid.prisms.len());
    }
}
