extern crate regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn get_passports() -> Vec<String> {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let mut data: Vec<String> = vec![];
    let mut current_passport = String::new();
    for line in lines {
        if let Ok(line) = line {
            if line.len() == 0 {
                data.push(current_passport);
                current_passport = String::new();
            } else {
                current_passport.push_str(&line);
                current_passport.push(' ');
            }
        }
    }

    // Add the last passport
    data.push(current_passport);
    
    return data;
}

fn first_part() {
    let data = get_passports();

    let valid_passports = data.iter().filter(|&passport| {
        passport.split_whitespace().filter(|&field| {
            let category = String::from(&field[0..3]);
            category == "byr" || category == "iyr" || category == "eyr" || category == "hgt" || category == "hcl" || category == "ecl" || category == "pid"
        }).count() == 7
    }).count();

    println!("{}", valid_passports);
}

fn second_part() {
    let data = get_passports();

    let hgt_re = Regex::new(r"(\d*)((?:in)|(?:cm))").unwrap();
    let hcl_re = Regex::new(r"#(\d|[a-f]){6}$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();

    let valid_passports = data.iter().filter(|&passport| {
        passport.split_whitespace().filter(|&field| {
            let category = String::from(&field[0..3]);
            let value = String::from(&field[4..]);

            match category.as_str() {
                "byr" => value.parse::<i32>().map_or(false, |year| year >= 1920 && year <= 2002),
                "iyr" => value.parse::<i32>().map_or(false, |year| year >= 2010 && year <= 2020),
                "eyr" => value.parse::<i32>().map_or(false, |year| year >= 2020 && year <= 2030),
                "hgt" => {
                    hgt_re.captures(&value)
                        .map_or(false, |captures| {
                            let measure = captures.get(1).map_or(Ok(-1), |m| m.as_str().parse::<i32>());
                            let measure = measure.map_or(-1, |i| i);
                            let unit = captures.get(2).map_or("", |m| m.as_str());
                            (unit == "cm" && measure >= 150 && measure <= 193) || (unit == "in" && measure >= 59 && measure <= 76)
                        })
                }
                "hcl" => hcl_re.is_match(&value),
                "ecl" => value == "amb" || value == "blu" || value == "brn" || value == "gry" || value == "grn" || value == "hzl" || value == "oth",
                "pid" => pid_re.is_match(&value),
                _ => false
            }
        }).count() == 7
    }).count();

    println!("{}", valid_passports);
}

fn main() {
    println!("--- Part One ---");
    first_part();
    println!("--- Part Two ---");
    second_part();
}