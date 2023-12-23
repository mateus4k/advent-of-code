use std::fs;

fn main() {
    parse();

    // println!("Part 1: {}", part_1());
    // println!("Part 2: {}", part_2());
}

fn parse() {
    let input = fs::read_to_string("../test.txt").expect("error while reading file");

    println!("{}", input);
}

#[allow(dead_code)]
fn part_1() {}

#[allow(dead_code)]
fn part_2() {}
