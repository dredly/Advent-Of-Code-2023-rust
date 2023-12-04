use std::{fs::File, io::Read, collections::{HashMap, HashSet}};

pub fn part1_answer() {
    let answer = sum_of_calibration_values("./inputs/day1/part1.txt".to_string());
    println!("Answer for day 1 part 1 = {}", answer);
}

pub fn part2_answer() {
    let answer = sum_of_calibration_values_part2("./inputs/day1/part1.txt".to_string());
    println!("Answer for day 1 part 2 = {}", answer);
}

fn sum_of_calibration_values(input_file_path: String) -> u32 {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents.split("\n")
        .map(|line| calibration_value(line))
        .sum()
}

fn calibration_value(line: &str) -> u32 {
    let first = line.chars()
        .find_map(|c| c.to_digit(10))
        .expect("Expected to find at least one digit in line");
    let last = line.chars()
        .rev()
        .find_map(|c| c.to_digit(10))
        .expect("Expected to find at least one digit in line");
    first * 10 + last
}

fn sum_of_calibration_values_part2(input_file_path: String) -> u32 {
    let lookup_table: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    contents.split("\n")
        .map(|line| calibration_value_part2(line, &lookup_table))
        .sum()
}

fn calibration_value_part2(line: &str, lookup_table: &HashMap<&str, u32>) -> u32 { 
    let first = first_calibration_value(line, lookup_table);
    let last = last_calibration_value(line, lookup_table);
    
    first * 10 + last
}

fn first_calibration_value(line: &str, lookup_table: &HashMap<&str, u32>) -> u32 {
    for (idx, c) in line.char_indices() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
        for l in get_key_lengths_ascending(lookup_table) {
            let end_idx = idx + l;
            if end_idx > line.len() {
                continue;
            }
            let substr = &line[idx..end_idx];
            if lookup_table.contains_key(substr) {
                return *lookup_table.get(substr).unwrap();
            }
        }
    }
    panic!("Expect to find a calibration value on every line")
}

fn last_calibration_value(line: &str, lookup_table: &HashMap<&str, u32>) -> u32 {
    for (idx, c) in line.char_indices().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
        for l in get_key_lengths_ascending(lookup_table) {
            let end_idx = idx + 1;
            if l > end_idx {
                continue;
            }
            let start_idx = end_idx - l;
            let substr = &line[start_idx..end_idx];
            if lookup_table.contains_key(substr) {
                return *lookup_table.get(substr).unwrap();
            }
        }
    }
    panic!("Expect to find a calibration value on every line")
} 

fn get_key_lengths_ascending(lookup_table: &HashMap<&str, u32>) -> Vec<usize> {
    let mut as_vec = Vec::from_iter(get_distinct_key_lengths(lookup_table).iter().map(|l| l.to_owned()));
    as_vec.sort();
    as_vec
}

fn get_distinct_key_lengths(lookup_table: &HashMap<&str, u32>) -> HashSet<usize> {
    lookup_table.keys().map(|k| k.len()).collect()
}