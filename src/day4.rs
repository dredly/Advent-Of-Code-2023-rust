use std::{collections::HashSet, fs::File, io::Read};

pub fn part1_answer() {
    let answer = total_points("./inputs/day4/part1.txt".to_string());
    println!("Answer for day 4 part 1 = {}", answer);
}

fn total_points(input_file_path: String) -> usize {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    contents.split("\n")
        .map(|line| Card::from_text(line))
        .map(|c| c.value())
        .sum()
}

struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn value(&self) -> usize {
        let num_winning: u32 = self.winning_numbers
            .intersection(&self.my_numbers)
            .into_iter()
            .count().try_into().unwrap();
        if num_winning < 2 {
            num_winning.try_into().unwrap()
        } else {
            2usize.pow(num_winning - 1)
        }
    }

    fn from_text(text: &str) -> Self {
        let (winning_part, my_part) = text.split_once(":").unwrap().1.split_once("|").unwrap();
        Card{
            winning_numbers: text_to_num_set(winning_part),
            my_numbers: text_to_num_set(my_part)
        }
    }
}

fn text_to_num_set(text: &str) -> HashSet<u32> {
    text
        .trim()
        .split_ascii_whitespace()
        .map(|substr| substr.parse::<u32>().unwrap())
        .collect()
}