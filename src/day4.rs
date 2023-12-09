use std::{collections::{HashSet, HashMap}, fs::File, io::Read};

pub fn part1_answer() {
    let answer = total_points("./inputs/day4/part1.txt".to_string());
    println!("Answer for day 4 part 1 = {}", answer);
}

pub fn part2_answer() {
    let answer = total_scratchcards_from_file("./inputs/day4/part1.txt".to_string());
    println!("Answer for day 4 part 2 = {}", answer);
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

#[derive(Clone)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn value(&self) -> usize {
        if self.num_winning() < 2 {
            self.num_winning().try_into().unwrap()
        } else {
            2usize.pow(self.num_winning() - 1)
        }
    }

    fn num_winning(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.my_numbers)
            .into_iter()
            .count().try_into().unwrap()
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

fn total_scratchcards_from_file(input_file_path: String) -> usize {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut counted_cards: HashMap<usize, (Card, usize)> = HashMap::new();

    let starting_cards = contents.split("\n")
        .map(|line| Card::from_text(line))
        .enumerate();

    for (idx, card) in starting_cards {
        counted_cards.insert(idx, (card, 1));
    }

    let mut line_num: usize = 0;
    while line_num < counted_cards.len() {
        let (card, count) = counted_cards.get_mut(&line_num).unwrap().clone();
        let start = line_num + 1;
        let end = start + card.num_winning() as usize;
        for i in start..end {
            counted_cards.entry(i).and_modify(|entry| *entry = (entry.0.clone(), entry.1 + count));
        }
        line_num += 1;
    }

    counted_cards
        .values()
        .map(|v| v.1)
        .sum()
}