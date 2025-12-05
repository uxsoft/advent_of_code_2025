pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

fn main() {
    let input = include_str!("day05/input.txt");
    let result = day05::p2::solve(input);
    println!("Part 2: {}", result);
}
