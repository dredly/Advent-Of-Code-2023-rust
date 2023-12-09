use std::{fs::File, io::Read};

pub fn part1_answer() {
    let answer = find_lowest_location("./inputs/day5/part1.txt".to_string());
    println!("Answer for day 5 part 1 = {}", answer);
}

fn find_lowest_location(input_file_path: String) -> isize {
    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let (seeds_part, almanac_part) = contents.split_once("\n\n").unwrap();
    let seeds = seeds_part
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|substr| substr.parse::<isize>().unwrap());
    let almanac = Almanac::from_text(almanac_part);
    seeds
        .map(|s| almanac.seed_to_soil(s))
        .min()
        .unwrap()
}

struct Almanac {
    maps: Vec<Map>
}

impl Almanac {
    fn seed_to_soil(&self, seed: isize) -> isize {
        let mut current_val = seed;
        for m in &self.maps {
            current_val = m.lookup(current_val);
        }
        current_val
    }

    fn from_text(text: &str) -> Self {
        let maps: Vec<Map> = text
            .split("\n\n")
            .map(|substr| Map::from_text(substr))
            .collect();
        Almanac { maps }
    }
}

struct Map {
    entries: Vec<MapEntry>
}

impl Map {
    fn lookup(&self, key: isize) -> isize {
        self.entries.iter()
            .find_map(|entry| entry.mapping(key))
            .unwrap_or(key)
    }

    fn from_text(text: &str) -> Self {
        let mut lines = text.lines();
        lines.next();
        let entries: Vec<MapEntry> = lines.map(|line| MapEntry::from_text(line)).collect();
        Map { entries }
    }
}

struct MapEntry {
    destination_range_start: isize,
    source_range_start: isize,
    range_length: isize,
}

impl MapEntry {
    fn mapping(&self, key: isize) -> Option<isize> {
        let offset = key - self.source_range_start;
        if offset >= 0 && offset < self.range_length {
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }

    fn from_text(text: &str) -> Self {
        let numbers: Vec<isize> = text
            .split_ascii_whitespace()
            .map(|substr| substr.parse::<isize>().unwrap())
            .collect();
        MapEntry {
            destination_range_start: numbers[0],
            source_range_start: numbers[1],
            range_length: numbers[2]
        }
    }
}
