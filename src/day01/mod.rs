pub mod p1;
pub mod p2;

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let minus = if line.starts_with("L") { -1 } else { 1 };
            let value = line.trim_start_matches(&['L', 'R']).parse::<i32>().unwrap();
            minus * value
        })
        .collect()
}
