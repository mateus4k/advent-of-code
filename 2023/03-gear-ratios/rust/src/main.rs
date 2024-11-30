use std::{collections::HashSet, fs};

fn main() {
    let (numbers, symbols, gears) = parse();

    println!("Part 1: {}", part_1(&numbers, &symbols));
    println!("Part 2: {}", part_2(&numbers, &gears));
}

fn parse() -> (Vec<PartNumber>, HashSet<(i64, i64)>, HashSet<(i64, i64)>) {
    let input = fs::read_to_string("../input.txt").expect("error while reading file");
    let mut numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: HashSet<(i64, i64)> = HashSet::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();

    let lines = &mut input.lines();
    let mut cur_number: Option<PartNumber> = None;

    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let is_number = ch.is_ascii_digit();

            if is_number {
                if let Some(ref mut num) = cur_number {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    cur_number = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    numbers.push(num);
                }
                if ch != '.' {
                    symbols.insert((row as i64, col as i64));
                    if ch == '*' {
                        gears.insert((row as i64, col as i64));
                    }
                }
            }
        }
    }

    return (numbers, symbols, gears);
}

fn part_1(numbers: &Vec<PartNumber>, symbols: &HashSet<(i64, i64)>) -> i64 {
    numbers
        .iter()
        .filter(|number| number.next_to_symbol(&symbols))
        .map(PartNumber::get_value)
        .sum::<i64>()
}

fn part_2(numbers: &Vec<PartNumber>, gears: &HashSet<(i64, i64)>) -> i64 {
    let mut total = 0;

    'next_gear: for gear in gears {
        let mut matches = Vec::new();

        for number in numbers {
            if number.points.contains(gear) {
                if matches.len() == 2 {
                    continue 'next_gear;
                }
                matches.push(number.value);
            }
        }

        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }

    return total;
}

#[derive(Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, column: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, column - 1),
            (row, column - 1),
            (row + 1, column - 1), // left hand side
            (row - 1, column),
            (row + 1, column), // above and below
            (row - 1, column + 1),
            (row, column + 1),
            (row + 1, column + 1), // right hand side
        ]);
        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, column: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        self.points.extend([
            (row - 1, column + 1),
            (row, column + 1),
            (row + 1, column + 1),
        ])
    }

    fn get_value(&self) -> i64 {
        self.value
    }

    fn next_to_symbol(&self, symbols: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(symbols).next().is_some()
    }
}
