use std::{fs::File, io::Read, collections::{HashSet, HashMap}};

use regex::{Regex, Match};

pub fn part1_answer() {
    let answer = sum_of_part_numbers_from_file("./inputs/day3/part1.txt".to_string());
    println!("Answer for day 3 part 1 = {}", answer);
}

pub fn part2_answer() {
    let answer = sum_of_gear_ratios_from_file("./inputs/day3/part1.txt".to_string());
    println!("Answer for day 3 part 2 = {}", answer);
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

fn special_char_indices(schematic: &str) -> HashSet<usize> {
    schematic
        .match_indices(|c: char| !c.is_digit(10) && c != '.')
        .map(|m| m.0)
        .collect()
}

fn adjacent_indices(special_char_indices: &HashSet<usize>, width: usize, height: usize) -> HashSet<usize> {
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

fn sum_of_gear_ratios_from_file(input_file_path: String) -> u32 {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let width = contents.lines().next().unwrap().len();
    sum_of_gear_ratios(&contents.as_str().replace("\n", ""), width) 
}

fn sum_of_gear_ratios(schematic: &str, width: usize) -> u32 {
    let height: usize = schematic.len() / width;
    let gear_indices: HashSet<usize> = schematic.match_indices('*')
        .map(|m| m.0)
        .collect();
    let re = Regex::new(r"\d+").unwrap();
    let gear_adjacencies = gear_adjacencies(&gear_indices, width, height);
    let gear_adjacent_numbers: Vec<_> = re.find_iter(schematic)
        .filter_map(|m| adjacent_to(&m, &gear_adjacencies))
        .collect();
    let mut sum: u32 = 0;
    for gan in &gear_adjacent_numbers {
        let adjacent_to_same_gear: Vec<GearAdjacentNumber> = gear_adjacent_numbers.iter()
            .filter(|other_gan| other_gan.gear_idx == gan.gear_idx)
            .map(|gan| gan.clone())
            .collect();
        if adjacent_to_same_gear.len() == 2 {
            let gear_ratio: u32 = adjacent_to_same_gear.iter()
                .map(|gan| gan.number)
                .product();
            sum += gear_ratio;
        }
    }
    // to account for double counting
    sum / 2
}

fn gear_adjacencies(gear_indices: &HashSet<usize>, width: usize, height: usize) -> HashMap<&usize, HashSet<usize>> {
    let mut adjacencies: HashMap<&usize, HashSet<usize>> = HashMap::new();
    for gi in gear_indices {
        let mut adjacent_to_gear = HashSet::new();
        // to the left
        if gi % width != 0 && gi > &0 {
            adjacent_to_gear.insert(gi - 1);
        }
        // to the right
        if (gi + 1) % width != 0 {
            adjacent_to_gear.insert(gi + 1);
        }
        // above
        if gi >= &width {
            adjacent_to_gear.insert(gi - width);
        }
        // below
        if gi + width < width * height {
            adjacent_to_gear.insert(gi + width);
        }
        // top left
        if gi >= &(width + 1) && gi % width != 0 {
            adjacent_to_gear.insert(gi - (width + 1));
        }
        // top right
        if gi >= &(width - 1) && (gi + 1) % width != 0 {
            adjacent_to_gear.insert(gi - (width - 1));
        }
        // bottom left
        if gi + (width - 1) < width * height && gi % width != 0 {
            adjacent_to_gear.insert(gi + (width - 1));
        }
        // bottom right
        if (gi + width + 1) < width * height && (gi + 1) % width != 0 {
            adjacent_to_gear.insert(gi + width + 1);
        }
        adjacencies.insert(gi, adjacent_to_gear);
    }
    adjacencies
}

fn adjacent_to(m: &Match, gear_adjacencies: &HashMap<&usize, HashSet<usize>>) -> Option<GearAdjacentNumber> {
    gear_adjacencies.iter()
        .find(|ga| is_adjacent(m.range(), ga.1))
        .map(|ga| ga.0)
        .map(|gi| GearAdjacentNumber{gear_idx: **gi, number: m.as_str().parse::<u32>().unwrap()})
}

#[derive(Clone, Copy)]
struct GearAdjacentNumber {
    gear_idx: usize,
    number: u32,
}