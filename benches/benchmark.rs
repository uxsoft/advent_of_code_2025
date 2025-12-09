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

#[divan::bench(max_time = 1)]
fn day4() {
    let _ = aoc::day04::p2::solve(divan::black_box(include_str!("../src/day04/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day5() {
    let _ = aoc::day05::p2::solve(divan::black_box(include_str!("../src/day05/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day6() {
    let _ = aoc::day06::p2::solve(divan::black_box(include_str!("../src/day06/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day7() {
    let _ = aoc::day07::p2::solve(divan::black_box(include_str!("../src/day07/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day8() {
    let _ = aoc::day08::p2::solve(divan::black_box(include_str!("../src/day08/input.txt")));
}
