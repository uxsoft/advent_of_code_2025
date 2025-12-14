use super::{Line2D, Point2D, Polygon2D, Rectangle2D};
use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    let points = super::parse(input);
    let polygon = super::Polygon2D::new(points.clone());

    // TODO optimisation - sort by area descending and find() instead of filter & max
    let max_area = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| super::Rectangle2D::new(points[i], points[j]))
        .sorted_by_cached_key(|rect| usize::MAX - rect.area())
        .find(|rect| !polygon.intersects(rect))
        .expect("No valid rectangle found");

    max_area.area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(24, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(1410501884, result);
    }
}
