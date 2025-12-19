pub fn solve(input: &str) -> usize {
    let (shapes, cases) = parse_input(input);
    let count = cases
        .into_iter()
        .map(|case| {
            if is_trivially_possible(&case) {
                1
            } else if is_trivially_impossible(&shapes, &case) {
                0
            } else {
                panic!("Non-trivial case");
            }
        })
        .sum();

    count
}

struct Case {
    region: (u32, u32),
    shape_counts: Vec<u32>,
}

fn is_trivially_possible(case: &Case) -> bool {
    // Every shape fits into a 3x3 box
    // There is enough 3x3 boxes in the space
    let num_shapes = case.shape_counts.iter().sum();
    (case.region.0 / 3) * (case.region.1 / 3) >= num_shapes
}

fn is_trivially_impossible(shapes: &[u32], case: &Case) -> bool {
    // There aren't enough points in the area for all the points the trees would occupy
    let total_shape_area = case
        .shape_counts
        .iter()
        .enumerate()
        .map(|(i, &num)| shapes[i] * num)
        .sum();
    case.region.0 * case.region.1 < total_shape_area
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Case>) {
    let mut shape_sizes = Vec::new();
    let mut shape_lines = 0;
    let mut shape_size = 0;
    let mut cases = Vec::new();
    for line in input.lines() {
        if line.contains('x') {
            cases.push(parse_case(&line));
        } else if line.contains('.') || line.contains('#') {
            // ignore the shape, just count the squares
            shape_size += line.chars().filter(|&ch| ch == '#').count();
            shape_lines += 1;
            if shape_lines == 3 {
                shape_sizes.push(shape_size as u32);
                shape_size = 0;
                shape_lines = 0;
            }
        }
    }
    (shape_sizes, cases)
}

fn parse_case(line: &str) -> Case {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let [width, height] = parts[0]
        .trim_end_matches(':')
        .split('x')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();

    Case {
        region: (width, height),
        shape_counts: parts[1..].iter().map(|s| s.parse().unwrap()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(2, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(550, result);
    }
}
