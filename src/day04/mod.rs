use std::{collections::HashMap, fmt::Display};

pub mod p1;
pub mod p2;

#[derive(Debug)]
pub enum Cell {
    Paper,
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Paper => write!(f, "@"),
            Cell::Empty => write!(f, "."),
        }
    }
}

impl Cell {
    pub fn parse(c: char) -> Self {
        match c {
            '@' => Cell::Paper,
            _ => Cell::Empty,
        }
    }

    pub fn is_paper(&self) -> bool {
        matches!(self, Cell::Paper)
    }
}

#[derive(Debug)]
pub struct Grid {
    rows: Vec<Vec<Cell>>,
    dim_x: usize,
    dim_y: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let rows: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| line.chars().map(Cell::parse).collect())
            .collect();

        let dim_x = rows[0].len();
        let dim_y = rows.len();

        Grid { rows, dim_x, dim_y }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.rows.get(y)?.get(x)
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.rows[y][x] = cell;
    }

    pub fn adjacent_coords4(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();

        if x > 0 {
            coords.push((x - 1, y));
        }
        if x < self.dim_x - 1 {
            coords.push((x + 1, y));
        }
        if y > 0 {
            coords.push((x, y - 1));
        }
        if y < self.dim_y - 1 {
            coords.push((x, y + 1));
        }

        coords
    }
    pub fn adjacent_coords8(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();

        if x > 0 {
            coords.push((x - 1, y));
        }
        if x < self.dim_x - 1 {
            coords.push((x + 1, y));
        }
        if y > 0 {
            coords.push((x, y - 1));
        }
        if y < self.dim_y - 1 {
            coords.push((x, y + 1));
        }
        if x > 0 && y > 0 {
            coords.push((x - 1, y - 1));
        }
        if x < self.dim_x - 1 && y > 0 {
            coords.push((x + 1, y - 1));
        }
        if x > 0 && y < self.dim_y - 1 {
            coords.push((x - 1, y + 1));
        }
        if x < self.dim_x - 1 && y < self.dim_y - 1 {
            coords.push((x + 1, y + 1));
        }

        coords
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.dim_y)
            .into_iter()
            .flat_map(|y| (0..self.dim_x).into_iter().map(move |x| (x, y)))
    }

    pub fn highlight<'a>(
        &'a self,
        highlights: &'a HashMap<(usize, usize), char>,
    ) -> GridHighlight<'a> {
        GridHighlight {
            highlights,
            grid: &self,
        }
    }
}

pub struct GridHighlight<'a> {
    highlights: &'a HashMap<(usize, usize), char>,
    grid: &'a Grid,
}

impl Display for GridHighlight<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.grid.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(c) = self.highlights.get(&(x, y)) {
                    write!(f, "{c}")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
