use std::collections::HashMap;

use super::Graph;

pub fn count_paths<'a>(
    g: &'a Graph,
    cache: &mut HashMap<(&'a str, bool, bool), usize>,
    from: &'a str,
    to: &'a str,
    dac: bool,
    fft: bool,
) -> usize {
    g.adj
        .get(from)
        .unwrap()
        .iter()
        .map(move |n| {
            if let Some(cached_answer) = cache.get(&(n, dac, fft)) {
                *cached_answer
            } else {
                let answer = match *n {
                    n if n == to => {
                        if dac && fft {
                            1
                        } else {
                            0
                        }
                    }
                    "dac" => count_paths(g, cache, n, to, true, fft),
                    "fft" => count_paths(g, cache, n, to, dac, true),
                    _ => count_paths(g, cache, n, to, dac, fft),
                };
                cache.insert((n, dac, fft), answer);
                answer
            }
        })
        .sum()
}

pub fn solve(input: &str) -> usize {
    let graph = super::Graph::parse(input);
    let mut cache = HashMap::new();

    count_paths(&graph, &mut cache, "svr", "out", false, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example2.txt");
        let result = solve(input);
        assert_eq!(2, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(380961604031372, result);
    }
}
