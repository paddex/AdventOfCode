use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_totals(file: File) -> Vec<i32> {
    let mut current: Vec<i32> = Vec::new();
    let mut totals: Vec<i32> = Vec::new();

    for line in BufReader::new(file).lines() {
        let text = line.unwrap();
        if text == "" {
            let mut total = 0;
            for val in &current {
                total += val;
            }
            totals.push(total);
            current.clear();
            continue;
        }
        let val: i32 = text.parse().unwrap();
        current.push(val);
    }

    let mut total = 0;
    for val in &current {
        total += val;
    }
    totals.push(total);

    totals
}

fn main() {
    let path = "./input/input".to_string();

    let file = File::open(path).unwrap();

    let totals: Vec<i32> = calc_totals(file);

    let mut max: i32 = 0;
    for total in &totals {
        if total > &max {
            max = *total;
        }
    }
    println!("Max: {}", max);
}
