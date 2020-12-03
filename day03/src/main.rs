use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_collisions(right: usize, down: usize) -> usize {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not find file");
    let mut lines = io::BufReader::new(file).lines();

    let mut x_pos = 0;
    let mut y_pos = 0;
    let width = lines.next().unwrap().unwrap().len();
    lines.filter(|line| {
        y_pos += 1;
        if y_pos % down != 0 {
            return false
        }
        x_pos = (x_pos + right) % width;
        line.as_ref().unwrap().chars().nth(x_pos).unwrap() == '#'
    }).count()
}

fn first_part() {
    println!("{}", get_collisions(3, 1));
}

fn second_part() {
    let r1d1 = get_collisions(1, 1);
    let r3d1 = get_collisions(3, 1);
    let r5d1 = get_collisions(5, 1);
    let r7d1 = get_collisions(7, 1);
    let r1d2 = get_collisions(1, 2);

    println!("Right 1, down 1: {}", r1d1);
    println!("Right 3, down 1: {}", r3d1);
    println!("Right 5, down 1: {}", r5d1);
    println!("Right 7, down 1: {}", r7d1);
    println!("Right 1, down 2: {}", r1d2);
    println!("Multiplied together: {}", r1d1 * r3d1 * r5d1 * r7d1 * r1d2);
}

fn main() {
    println!("--- Part One ---");
    first_part();
    println!("--- Part Two ---");
    second_part();
}
