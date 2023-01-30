use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// static SCORE_MAP: HashMap<&str, i32> = HashMap::from([
//     ("A", 1),
//     ("B", 2),
//     ("C", 3),
//     ("X", 1),
//     ("Y", 2),
//     ("Z", 3)
// ]);

fn main() {
    let file = File::open("./data/input.txt".to_string()).unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        let string = line.unwrap();
        let choices = string.split(" ");
        let vec: Vec<&str> = choices.collect();
        println!("{:?}", &vec);
    }
}