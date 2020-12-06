use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn first_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    let mut party_set: HashSet<char> = HashSet::new();
    for line in lines {
        if let Ok(line) = line {
            if line.len() == 0 {
                sum += party_set.iter().count();
                party_set.clear();
                continue;
            }

            for c in line.chars() {
                party_set.insert(c);
            }
        }
    }
    sum += party_set.iter().count();
    println!("{}", sum);
}

fn second_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    let mut party_set: HashSet<char> = HashSet::new();
    let mut is_new_group = true;
    for line in lines {
        if let Ok(line) = line {
            if line.len() == 0 {
                sum += party_set.iter().count();
                party_set.clear();
                is_new_group = true;
                continue;
            }

            if is_new_group {
                party_set = line.chars().collect();
                is_new_group = false;
            } else {
                let person_set: HashSet<char> = line.chars().collect();
                party_set = party_set.intersection(&person_set).cloned().collect();
            }
        }
    }
    sum += party_set.iter().count();
    println!("{}", sum);
}

fn main() {
    println!("--- Part One ---");
    first_part();
    println!("--- Part Two ---");
    second_part();
}
