fn main() {
    divan::main();
}

#[path = "../src/main.rs"]
mod aoc;

#[divan::bench(max_time = 1)]
fn day1() {
    let _ = aoc::day01::p2::solve(divan::black_box(include_str!("../src/day01/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day2() {
    let _ = aoc::day02::p2::solve(divan::black_box(include_str!("../src/day02/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day3() {
    let _ = aoc::day03::p2::solve(divan::black_box(include_str!("../src/day03/input.txt")));
}
