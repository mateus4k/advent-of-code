use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("error while reading file");

    let result: i32 = input
        .split('\n')
        .filter_map(|line| {
            let mut first = String::new();
            let mut last = String::new();

            line.chars().for_each(|char| {
                if char.to_string().parse::<i32>().is_err() {
                    return;
                }

                last = char.to_string();

                if first.is_empty() {
                    first = char.to_string();
                }
            });

            format!("{first}{last}").parse::<i32>().ok()
        })
        .sum::<i32>();

    println!("Result: {}", result);
}
