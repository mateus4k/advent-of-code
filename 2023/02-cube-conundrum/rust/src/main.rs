use std::fs;

fn main() {
    let game_list: Vec<Vec<Turn>> = parse();

    let valid_games = part_1(&game_list);
    let power_sum = part_2(&game_list);

    println!("Part 1: {}", valid_games);
    println!("Part 2: {}", power_sum);
}

fn part_1(game_list: &Vec<Vec<Turn>>) -> usize {
    let mut total = 0;

    'next_game: for (index, game) in game_list.iter().enumerate() {
        for turn in game {
            if !turn.is_valid() {
                continue 'next_game;
            }
        }

        total += index + 1;
    }

    total
}

fn part_2(game_list: &Vec<Vec<Turn>>) -> usize {
    let mut power_sum = 0;

    for game in game_list {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for turn in game {
            red = red.max(turn.red);
            green = green.max(turn.green);
            blue = blue.max(turn.blue);
        }
        power_sum += red * green * blue;
    }

    power_sum
}

fn parse() -> Vec<Vec<Turn>> {
    let input = fs::read_to_string("../input.txt").expect("error while reading file");
    let lines = input.trim().split('\n');
    let mut game_list = Vec::new();

    for line in lines {
        let (_, turns) = line.split_once(":").expect("fail to split turns");
        let turns = turns.split(";").collect::<Vec<_>>();

        let mut turn_list = Vec::new();

        for turn in turns {
            let cubes = turn.split(", ").collect::<Vec<_>>();
            let mut turn = Turn::default();
            for cube in cubes {
                let (amount, color) = cube.trim().split_once(" ").expect("fail to split cube");
                let amount: usize = amount.parse().expect("fail to parse cube amount");
                // println!("am: {} - cl: {}\n", amount, color);
                match &color[0..1] {
                    "r" => turn.red = amount,
                    "g" => turn.green = amount,
                    "b" => turn.blue = amount,
                    _ => panic!("fail to parse input"),
                }
            }

            turn_list.push(turn);
        }

        game_list.push(turn_list);
    }

    game_list
}

#[derive(Debug, Default)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

impl Turn {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
