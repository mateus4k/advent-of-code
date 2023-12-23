use std::{fs, ops::Range};

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, multispace1, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = fs::read_to_string("../input.txt").expect("error while reading file");

    let (_, part_1) = part_1(&input).unwrap();
    println!("Part 1: {}", part_1);

    let (_, part_2) = part_2(&input).unwrap();
    println!("Part 2: {}", part_2);
}

fn part_1(input: &str) -> IResult<&str, String> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(multispace1, complete::u64))
        .parse(input)?;

    let (_, maps) = many1(seed_map)(input)?;

    let locations = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
        .collect::<Vec<u64>>();

    let result = locations.iter().min().unwrap().to_string();

    Ok((input, result))
}

fn part_2(input: &str) -> IResult<&str, u64> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, tag(" "), complete::u64)
                .map(|(start, offset)| start..(start + offset)),
        ))
        .parse(input)?;

    let (_, maps) = many1(seed_map)(input)?;

    let result = seeds
        .into_par_iter()
        .flat_map(|range| range.clone())
        .map(|seed| maps.iter().fold(seed, |seed, map| map.translate(seed)))
        .min()
        .unwrap();

    Ok((input, result))
}

fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}
