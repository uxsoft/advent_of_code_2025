use std::collections::HashMap;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use super::{Button, Joltage, Problem, State, parse};

// Todo change to big operations on usize
pub fn press_button(state: &mut Joltage, button: &Button, target: &Joltage) -> bool {
    for e in button.connections.iter().cloned() {
        state.s[e] = state.s[e] + 1;
        if state.s[e] > target.s[e] {
            return false;
        }
    }

    return true;
}

pub fn solve_problem(problem: &Problem) -> usize {
    let mut queue = vec![Joltage::initial()];
    let mut cache: HashMap<Joltage, usize> = HashMap::new();
    cache.insert(Joltage::initial(), 0);

    while let Some(current_item) = queue.pop() {
        if let Some(&current_cost) = cache.get(&current_item) {
            for button in &problem.buttons {
                let mut new_state = current_item.clone();
                if !press_button(&mut new_state, button, &problem.joltage_req) {
                    continue;
                }

                if new_state == problem.joltage_req {
                    return current_cost + 1;
                }

                match cache.get(&new_state) {
                    Some(&cost) if cost > current_cost + 1 => {
                        // new_state already exists, but we have a better path -> update & re-trigger
                        cache.insert(new_state.clone(), current_cost + 1);
                        queue.push(new_state);
                        //println!("Found a better path");
                    }
                    Some(_) => {
                        // new_state already exists, no need to update cost
                    }
                    None => {
                        // Completely a new state, explore further
                        cache.insert(new_state.clone(), current_cost + 1);
                        queue.push(new_state);
                    }
                }
            }
        }
    }

    *cache.get(&problem.joltage_req).unwrap_or(&usize::MAX)
}

pub fn solve(input: &str) -> usize {
    let problems = parse(input);

    problems.iter().map(solve_problem).sum()
}

pub fn solve_par(input: &str) -> usize {
    let problems = parse(input);

    problems
        .par_iter()
        .progress_count(problems.len() as u64)
        .map(solve_problem)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = include_str!("example.txt");
        let problems = super::parse(input);
        let result = solve_problem(problems.first().unwrap());
        assert_eq!(10, result);
    }

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(33, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(1, result);
    }
}
