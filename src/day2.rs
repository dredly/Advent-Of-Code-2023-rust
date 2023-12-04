use std::{collections::HashMap, fs::File, io::Read};

use regex::Regex;

pub fn part1_answer() {
    let answer = sum_of_possible_game_ids("./inputs/day2/part1.txt".to_string());
    println!("Answer for day 2 part 1 = {}", answer);
}

pub fn part2_answer() {
    let answer = sum_of_power_values("./inputs/day2/part1.txt".to_string());
    println!("Answer for day 2 part 2 = {}", answer);
}

fn sum_of_possible_game_ids(input_file_path: String) -> u32 {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let max_amounts: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let re = Regex::new(r"(\d+)\s(green|red|blue)").unwrap();

    contents.split("\n")
        .map(|line| possibility_value(line, &max_amounts, &re))
        .sum()
}

fn possibility_value(game: &str, max_amounts: &HashMap<&str, u32>, amount_regex: &Regex) -> u32 {
    // Returns the id of a game if it is possible, otherwise returns 0
    let spl: Vec<&str> = game.split(":").collect();
    let amounts = spl[1];
    for (_, [amount, colour]) in amount_regex.captures_iter(amounts).map(|c| c.extract()) {
        if max_amounts.get(colour).unwrap() < &amount.parse::<u32>().unwrap() {
            return 0;
        }
    }
    let game_info = spl[0];
    let game_id = game_info.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
    return game_id
}

fn sum_of_power_values(input_file_path: String) -> u32 {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let re = Regex::new(r"(\d+)\s(green|red|blue)").unwrap();

    contents.split("\n")
        .map(|line| power_value(line, &re))
        .sum()
}

fn power_value(game: &str, amount_regex: &Regex) -> u32 {
    let spl: Vec<&str> = game.split(":").collect();
    let amounts = spl[1];
    let mut min_required: HashMap<&str, u32> = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);
    for (_, [amount, colour]) in amount_regex.captures_iter(amounts).map(|c| c.extract()) {
        if &amount.parse::<u32>().unwrap() > min_required.get(colour).unwrap() {
            min_required.insert(colour, amount.parse::<u32>().unwrap());
        }
    }
    min_required.values().product()
}