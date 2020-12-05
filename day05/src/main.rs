use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_seat_ids() -> impl Iterator<Item = i32> {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    lines.map(|line| {
        let mut seat_id = -1;
        if let Ok(line) = line {
            let row_bsp = &line[0..7];
            let col_bsp = &line[7..];
            let mut row = (0, 128);
            let mut col = (0, 8);

            for c in row_bsp.chars() {
                let half = (row.1 - row.0) / 2;
                if c == 'F' {
                    row.1 -= half;
                } else {
                    row.0 += half;
                }
            }

            for c in col_bsp.chars() {
                let half = (col.1 - col.0) / 2;
                if c == 'L' {
                    col.1 -= half;
                } else {
                    col.0 += half;
                }
            }

            seat_id = row.0 * 8 + col.0;
        }
        seat_id
    })
}

fn first_part() {
    let highest = get_seat_ids().max();

    if let Some(highest) = highest {
        println!("{}", highest);
    }
}

fn second_part() {
    let mut seat_ids: Vec<i32> = get_seat_ids().collect();
    seat_ids.sort();
    for (i, id) in seat_ids.iter().enumerate() {
        if i == seat_ids.len() - 1 {
            break;
        }

        let next = seat_ids[i + 1];
        if next - id > 1 {
            println!("{}", id + 1);
            break;
        }
    }
}

fn main() {
    println!("--- Part One ---");
    first_part();
    println!("--- Part Two ---");
    second_part();
}
