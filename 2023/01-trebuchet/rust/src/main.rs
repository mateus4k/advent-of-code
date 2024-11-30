use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("error while reading file");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &String) -> u32 {
    let result: u32 = input
        .trim()
        .split('\n')
        .filter_map(|line| {
            let mut first = String::new();
            let mut last = String::new();

            line.chars().for_each(|char| {
                if char.to_string().parse::<u32>().is_err() {
                    return;
                }

                last = char.to_string();

                if first.is_empty() {
                    first = char.to_string();
                }
            });

            format!("{first}{last}").parse::<u32>().ok()
        })
        .sum::<u32>();

    result
}

fn part_2(input: &String) -> u32 {
    let result = input
        .trim()
        .split('\n')
        .map(|line| {
            let mut iter = (0..line.len()).filter_map(|index| match &line[index..] {
                line if line.starts_with("one") => Some(1),
                line if line.starts_with("two") => Some(2),
                line if line.starts_with("three") => Some(3),
                line if line.starts_with("four") => Some(4),
                line if line.starts_with("five") => Some(5),
                line if line.starts_with("six") => Some(6),
                line if line.starts_with("seven") => Some(7),
                line if line.starts_with("eight") => Some(8),
                line if line.starts_with("nine") => Some(9),
                line => line.chars().next().unwrap().to_digit(10),
            });

            let first = iter.next().expect("should be a number");

            match iter.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();

    result
}
