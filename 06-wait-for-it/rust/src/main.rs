use nom::{
    bytes::complete::is_not,
    character::complete::{self, digit1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("error while reading file");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn parse_separated_nums(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn join_separated_nums(input: &str) -> IResult<&str, u64> {
    is_not("0123456789")
        .precedes(separated_list1(space1, digit1).map(|list| list.join("").parse::<u64>().unwrap()))
        .parse(input)
}

fn part_1(input: &str) -> usize {
    let (_, (times, distances)) =
        separated_pair(parse_separated_nums, line_ending, parse_separated_nums)
            .parse(input)
            .unwrap();

    let races = times
        .into_iter()
        .zip(distances)
        .collect::<Vec<(u32, u32)>>();

    races
        .into_iter()
        .map(|(time, record_distance)| {
            (0..time)
                .filter_map(|speed| {
                    let reached_distance = (time - speed) * speed;

                    (reached_distance > record_distance).then_some(reached_distance)
                })
                .count()
        })
        .product::<usize>()
}

fn part_2(input: &str) -> usize {
    let (_, (time, record_distance)) =
        separated_pair(join_separated_nums, line_ending, join_separated_nums)
            .parse(input)
            .unwrap();

    (0..time)
        .filter_map(|speed| {
            let reached_distance = (time - speed) * speed;

            (reached_distance > record_distance).then_some(reached_distance)
        })
        .count()
}
