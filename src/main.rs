pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

fn main() {
    let input = include_str!("day10/example.txt");
    let result = day10::p2_lp::solve(input);
    println!("Part 2: {}", result);
}
