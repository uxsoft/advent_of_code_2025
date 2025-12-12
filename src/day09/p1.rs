use super::Rectangle2D;

pub fn solve(input: &str) -> usize {
    let points = super::parse(input);

    let max_area = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| Rectangle2D::new(points[i], points[j]))
        .map(|rect| rect.area())
        .max()
        .unwrap();

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(50, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(4777816465, result);
    }
}
