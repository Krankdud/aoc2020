extern crate regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn insert_bags(bag: &str, bag_map: &HashMap<String, Vec<String>>, bag_set: &mut HashSet<String>) {
    if let Some(bags) = bag_map.get(bag) {
        for bag in bags {
            bag_set.insert(bag.clone());
            insert_bags(&bag, bag_map, bag_set);
        }
    }
}

fn first_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let bag_re = Regex::new(r"\d (?P<bag>[a-z ]*bag)").unwrap();
    let mut bag_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        if let Ok(line) = line {
            let rule: Vec<&str> = line.split("s contain ").collect();
            for caps in bag_re.captures_iter(&rule[1]) {
                let key = String::from(&caps["bag"]);
                bag_map.entry(key).or_insert(vec![]).push(String::from(rule[0]));
            }
        }
    }

    let mut bag_set = HashSet::new();
    insert_bags("shiny gold bag", &bag_map, &mut bag_set);
    println!("{}", bag_set.len());
}

struct Bag {
    count: usize,
    name: String
}

fn get_total_bags(bag_name: &str, bag_map: &HashMap<String, Vec<Bag>>) -> usize {
    let mut sum = 0;
    if let Some(bags) = bag_map.get(bag_name) {
        for bag in bags {
            sum += bag.count + bag.count * get_total_bags(&bag.name, bag_map);
        }
    }
    return sum;
}

fn second_part() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let bag_re = Regex::new(r"(?P<count>\d+) (?P<bag>[a-z ]*bag)").unwrap();
    let mut bag_map: HashMap<String, Vec<Bag>> = HashMap::new();

    for line in lines {
        if let Ok(line) = line {
            let rule: Vec<&str> = line.split("s contain ").collect();
            let bags: Vec<Bag> = bag_re.captures_iter(&rule[1])
                .map(|caps| {
                    let count = caps["count"].parse::<usize>().unwrap();
                    let name = String::from(&caps["bag"]);
                    Bag { count, name }
                }).collect();
            bag_map.insert(String::from(rule[0]), bags);
        }
    }

    println!("{}", get_total_bags("shiny gold bag", &bag_map));
}

fn main() {
    println!("--- Part One ---");
    first_part();
    println!("--- Part Two ---");
    second_part();
}
