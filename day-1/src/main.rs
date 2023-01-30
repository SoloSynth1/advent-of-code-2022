use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::vec::Vec;

fn main() {
    let reader = BufReader::new(
        File::open("./data/input.txt".to_string()).unwrap()
    );
    let lines = reader.lines();

    let mut total = 0;
    let mut totals = Vec::new();
    for line in lines {
        let content = &line.unwrap();
        if content == "" {
            totals.push(total);
            total = 0
        } else {
            total += content.parse::<i32>().unwrap()
        };
    }
    totals.sort();
    let len_total = totals.len();
    let top1 = totals[len_total-1];
    let sum_top3: &i32 = &totals[len_total-3..len_total].iter().sum();

    println!("Most calories: {}", top1);
    println!("Combined calories from top-3 elves: {}", sum_top3);
}