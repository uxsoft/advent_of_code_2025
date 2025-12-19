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
pub mod day11;
pub mod day12;

fn main() {
    let input = include_str!("day11/input.txt");
    let result = day11::p2::solve(input);
    println!("Part 2: {}", result);
}
