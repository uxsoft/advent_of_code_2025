pub mod p1;
pub mod p2;

fn parse(input: &str) -> Vec<std::ops::RangeInclusive<usize>> {
    input
        .replace(['\r', '\n'], "")
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .map(|(from, to)| {
                    let from = from.parse().unwrap();
                    let to = to.parse().unwrap();
                    from..=to
                })
                .unwrap()
        })
        .collect()
}
