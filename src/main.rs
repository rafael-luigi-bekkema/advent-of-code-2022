#![feature(hash_drain_filter)]
mod day01;
mod day02;
mod day03;

fn main() {
    println!("Advent of Code 2022!");

    println!("day 1a: {}", day01::a());
    println!("day 1b: {}", day01::b());

    println!("day 2a: {}", day02::a());
    println!("day 2b: {}", day02::b());

    println!("day 3a: {}", day03::a());
    println!("day 3b: {}", day03::b());
}
