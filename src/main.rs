pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

fn main() {
    let input = include_str!("day08/input.txt");
    let result = day08::p2::solve(input);
    println!("Part 2: {}", result);
}
