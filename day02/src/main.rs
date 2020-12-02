use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn first_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not find file");
    let lines = io::BufReader::new(file).lines();

    let mut valid_passwords = 0;
    for line in lines {
        if let Ok(line) = line {
            let entry: Vec<&str> = line.split_whitespace().collect();
            let range: Vec<&str> = entry[0].split('-').collect();
            let min = range[0].parse::<i32>().unwrap();
            let max = range[1].parse::<i32>().unwrap();
            let chr = entry[1].chars().next().unwrap();

            let mut count = 0;
            for c in entry[2].chars() {
                if c == chr {
                    count += 1;
                    if count > max {
                        break;
                    }
                }
            }
            if count >= min && count <= max {
                valid_passwords += 1;
            }
        }
    }
    println!("{}", valid_passwords);
}

fn second_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not find file");
    let lines = io::BufReader::new(file).lines();

    let mut valid_passwords = 0;
    for line in lines {
        if let Ok(line) = line {
            let entry: Vec<&str> = line.split_whitespace().collect();
            let range: Vec<&str> = entry[0].split('-').collect();
            let pos1 = range[0].parse::<usize>().unwrap();
            let pos2 = range[1].parse::<usize>().unwrap();
            let chr = entry[1].chars().next().unwrap();

            let c1 = entry[2].chars().nth(pos1 - 1).unwrap();
            let c2 = entry[2].chars().nth(pos2 - 1).unwrap();
            
            if (c1 == chr) ^ (c2 == chr) {
                valid_passwords += 1;
            }
        }
    }
    println!("{}", valid_passwords);
}

fn main() {
    println!("--- Part One ---");
    first_part();
    println!("--- Part Two ---");
    second_part();
}
