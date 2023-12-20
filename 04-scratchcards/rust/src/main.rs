use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("../input.txt").expect("error while reading file");
    println!("Part 1: {}", part_1(&input)); // 32001
    println!("Part 2: {}", part_2(&input)); // 5037841
}

fn part_1(input: &str) -> u32 {
    let mut total_points: u32 = 0;

    for line in input.lines() {
        let (_, line) = line.split_once(":").unwrap();
        let (winning_numbers, my_numbers) = line.split_once("|").unwrap();

        let winning_numbers = read_numbers(winning_numbers);
        let my_numbers = read_numbers(my_numbers);
        let my_winning_numbers = winning_numbers
            .intersection(&my_numbers)
            .collect::<Vec<_>>();

        let points = if my_winning_numbers.is_empty() {
            0
        } else {
            my_winning_numbers.iter().skip(1).fold(1, |acc, _| acc * 2)
        };

        total_points += points;
    }

    total_points
}

fn part_2(input: &str) -> usize {
    let mut cards_quantity: HashMap<usize, usize> = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let (_, line) = line.split_once(":").unwrap();
        let (winning_numbers, my_numbers) = line.split_once("|").unwrap();

        let winning_numbers = read_numbers(winning_numbers);
        let my_numbers = read_numbers(my_numbers);
        let winning_numbers_count = winning_numbers.intersection(&my_numbers).count();

        // Initialize current card
        let current_card_quantity = *cards_quantity.entry(index + 1).or_insert(1);
        // Increment next cards
        for next_line_index in index..(index + winning_numbers_count) {
            let key = next_line_index + 2;
            let next_card_quantity = *cards_quantity.entry(key).or_insert(1);
            cards_quantity.insert(key, next_card_quantity + current_card_quantity);
        }
    }

    cards_quantity.into_values().sum()
}

fn read_numbers(input: &str) -> HashSet<u32> {
    let mut result = HashSet::new();

    for item in input.trim().split(' ') {
        if let Ok(num) = item.parse::<u32>() {
            result.insert(num);
        }
    }

    result
}
