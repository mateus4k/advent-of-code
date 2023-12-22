use std::fs;

fn main() {
    let input = fs::read_to_string("../test.txt").expect("error while reading file");
    parse(&input);

    // println!("Part 1: {}", part_1(&numbers, &symbols));
    // println!("Part 2: {}", part_2(&numbers, &gears));
}

fn parse(input: &str) {
    let seeds: &str = input.lines().next().unwrap().split_once(":").unwrap().1;
    let seeds: Vec<i64> = seeds
        .trim()
        .split(' ')
        .map(|seed| seed.trim().parse().unwrap())
        .collect();

    println!("{:?}", seeds);
}

fn part_1() {}

fn part_2() {}
