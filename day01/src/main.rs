use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn first_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let mut num_set: HashSet<i32> = HashSet::new();

    for line in lines {
        if let Ok(num) = line {
            let num = num.parse::<i32>().unwrap();
            if num_set.contains(&num) {
                let other = 2020 - num;
                println!("{}", num * other);
                return;
            }
            num_set.insert(2020 - num);
        }
    }
}

fn second_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let mut num_set: HashSet<i32> = HashSet::new();
    let mut num_vec: Vec<i32> = vec![];

    for line in lines {
        if let Ok(num) = line {
            let num = num.parse::<i32>().unwrap();
            num_set.insert(2020 - num);
            num_vec.push(num);
        }
    }

    for (i, m) in num_vec.iter().enumerate() {
        for (j, n) in num_vec.iter().enumerate() {
            if i == j { continue; }

            let sum = m + n;
            if num_set.contains(&sum) {
                let other = 2020 - sum;
                println!("{}", m * n * other);
                return;
            }
        }
    }
}

fn main() {
    println!("--- Part One ---");
    first_part();
    println!("--- Part Two ---");
    second_part();
}
