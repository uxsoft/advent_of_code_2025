use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;

pub mod p1;
pub mod p2;

#[derive(Clone, Copy)]
pub struct Point2D {
    x: usize,
    y: usize,
}

impl Debug for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point2D {
    pub fn new(x: usize, y: usize) -> Self {
        Point2D { x, y }
    }
    pub fn parse(input: &str) -> Self {
        let (x_str, y_str) = input.split_once(',').unwrap();
        let x = x_str.parse().unwrap();
        let y = y_str.parse().unwrap();
        Point2D { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Line2D {
    from: Point2D,
    to: Point2D,
}

impl Line2D {
    pub fn new(from: Point2D, to: Point2D) -> Result<Line2D, String> {
        if from.x == to.x || from.y == to.y {
            Ok(Line2D { from, to })
        } else {
            Err(format!("Points are not in one line {}, {}", from, to))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle2D {
    from: Point2D,
    to: Point2D,
}

impl Rectangle2D {
    pub fn new(from: Point2D, to: Point2D) -> Self {
        Rectangle2D { from, to }
    }

    pub fn area(&self) -> usize {
        let width = self.from.x.abs_diff(self.to.x) + 1;
        let height = self.from.y.abs_diff(self.to.y) + 1;

        width * height
    }

    pub fn intersects(&self, line: &Line2D) -> bool {
        let line_top = line.from.y.min(line.to.y);
        let line_bottom = line.from.y.max(line.to.y);
        let line_left = line.from.x.min(line.to.x);
        let line_right = line.from.x.max(line.to.x);
        let rect_top = self.from.y.min(self.to.y);
        let rect_bottom = self.from.y.max(self.to.y);
        let rect_left = self.from.x.min(self.to.x);
        let rect_right = self.from.x.max(self.to.x);

        let before_rect = rect_left >= line_right;
        let after_rect = rect_right <= line_left;
        let above_rect = rect_top >= line_bottom;
        let below_rect = rect_bottom <= line_top;

        // Does not intersect if before after above or below
        !(before_rect || after_rect || above_rect || below_rect)
    }
}

#[derive(Debug, Clone)]
pub struct Polygon2D {
    pub lines: Vec<Line2D>,
}

impl Polygon2D {
    pub fn new(points: Vec<Point2D>) -> Self {
        let lines = points
            .into_iter()
            .circular_tuple_windows()
            .map(|(a, b)| Line2D::new(a, b).unwrap())
            .collect();

        Self { lines }
    }

    pub fn intersects(&self, other: &Rectangle2D) -> bool {
        self.lines.iter().any(|line| other.intersects(line))
    }
}

pub fn parse(input: &str) -> Vec<Point2D> {
    input.lines().map(Point2D::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_intersect() {
        let rect = Rectangle2D::new(Point2D::new(11, 1), Point2D::new(2, 5));

        let line = Line2D::new(Point2D::new(2, 3), Point2D::new(7, 3)).unwrap();
        assert!(rect.intersects(&line));

        let line = Line2D::new(Point2D::new(7, 3), Point2D::new(7, 1)).unwrap();
        assert!(rect.intersects(&line));
    }

    #[test]
    fn test_area() {
        let p1 = Point2D::new(2, 5);
        let p2 = Point2D::new(11, 1);
        let r = Rectangle2D::new(p1, p2);
        assert_eq!(50, r.area());
    }
}
