use std::collections::HashMap;

use super::{Button, Problem, State, parse};

// Todo change to big operations on usize
pub fn press_button(state: &mut State, button: &Button) {
    for e in &button.connections {
        state.s[*e] = !state.s[*e];
    }
}

pub fn solve_problem(problem: &Problem) -> usize {
    let mut queue = vec![State::initial()];
    let mut cache: HashMap<State, usize> = HashMap::new();
    cache.insert(State::initial(), 0);

    while let Some(current_item) = queue.pop() {
        if let Some(&current_cost) = cache.get(&current_item) {
            for button in &problem.buttons {
                let mut new_state = current_item.clone();
                press_button(&mut new_state, button);

                match cache.get(&new_state) {
                    Some(&cost) if cost > current_cost + 1 => {
                        // new_state already exists, but we have a better path -> update & re-trigger
                        cache.insert(new_state.clone(), current_cost + 1);
                        queue.push(new_state);
                        //println!("Found a better path");
                    }
                    Some(&cost) => {
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

    *cache.get(&problem.final_state).unwrap_or(&usize::MAX)
}

pub fn solve(input: &str) -> usize {
    let problems = parse(input);

    // let first = problems.first().unwrap();
    // let first_sol = solve_problem(first);
    // println!("First problem: {:?}", first_sol);

    problems.iter().map(solve_problem).sum()
    //first_sol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solve(input);
        assert_eq!(7, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = solve(input);
        assert_eq!(498, result);
    }
}
