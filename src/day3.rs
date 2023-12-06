use std::{fs::File, io::Read, collections::HashSet};

use regex::Regex;

pub fn part1_answer() {
    let answer = sum_of_part_numbers_from_file("./inputs/day3/part1.txt".to_string());
    println!("Answer for day 3 part 1 = {}", answer);
}

fn sum_of_part_numbers_from_file(input_file_path: String) -> u32 {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");


    let width = contents.lines().next().unwrap().len();
    sum_of_part_numbers(&contents.as_str().replace("\n", ""), width) 
}

fn sum_of_part_numbers(schematic: &str, width: usize) -> u32 {
    let height: usize = schematic.len() / width;
    let special_char_indices = special_char_indices(schematic);
    let re = Regex::new(r"\d+").unwrap();
    let adjacent_indices = adjacent_indices(&special_char_indices, width, height);
    re.find_iter(schematic)
        .filter(|m| is_adjacent(m.range(), &adjacent_indices))
        .map(|m| m.as_str())
        .map(|s| s.parse::<u32>().unwrap())
        .sum()
}

fn special_char_indices(schematic: &str) -> Vec<usize> {
    schematic
        .match_indices(|c: char| !c.is_digit(10) && c != '.')
        .map(|m| m.0)
        .collect()
}

fn adjacent_indices(special_char_indices: &Vec<usize>, width: usize, height: usize) -> HashSet<usize> {
    let mut adjacent = HashSet::new();
    for sci in special_char_indices {
        // to the left
        if sci % width != 0 && sci > &0 {
            adjacent.insert(sci - 1);
        }
        // to the right
        if (sci + 1) % width != 0 {
            adjacent.insert(sci + 1);
        }
        // above
        if sci >= &width {
            adjacent.insert(sci - width);
        }
        // below
        if sci + width < width * height {
            adjacent.insert(sci + width);
        }
        // top left
        if sci >= &(width + 1) && sci % width != 0 {
            adjacent.insert(sci - (width + 1));
        }
        // top right
        if sci >= &(width - 1) && (sci + 1) % width != 0 {
            adjacent.insert(sci - (width - 1));
        }
        // bottom left
        if sci + (width - 1) < width * height && sci % width != 0 {
            adjacent.insert(sci + (width - 1));
        }
        // bottom right
        if (sci + width + 1) < width * height && (sci + 1) % width != 0 {
            adjacent.insert(sci + width + 1);
        }
    }
    adjacent
}

fn is_adjacent(r: core::ops::Range<usize>, adjacent_indices: &HashSet<usize>) -> bool {
    adjacent_indices.iter()
        .any(|idx| r.contains(idx))
}