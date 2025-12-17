use super::{Problem, parse};

fn solve_problem(p: &Problem) -> usize {
    use good_lp::*;

    // create variable per button (number of times it got pressed)
    let mut vars = variables!();
    let variables: Vec<Variable> = p
        .buttons
        .iter()
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    // minimize total presses
    let objective: Expression = variables.iter().sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    // for each jolt counter, desired value must equal the sum of presses of each button * (1/0) if the button is connected to that jolt counter
    for (j_index, &j_target) in p.joltage_req.s.iter().enumerate() {
        let mut expr = Expression::from(0.0);

        for btn in &p.buttons {
            // if button is relevant, add its press variable to the constraint
            if btn.connections.contains(&j_index) {
                expr += variables[btn.id];
            }
        }

        // sum of relevant presses == target joltage
        problem.add_constraint(expr.eq(j_target));
    }

    let solution = problem.solve().unwrap();

    variables
        .iter()
        .map(|v| solution.value(*v).round() as usize)
        .sum()
}

pub fn solve(input: &str) -> usize {
    let problems = super::parse(input);

    problems.iter().map(solve_problem).sum()
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
        assert_eq!(17133, result);
    }
}
