use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::string::ToString;
use std::collections::VecDeque;

fn main() {
    let file_path = "./data/input.txt";
    let lines = read_lines(file_path);

    let mut a = false;
    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    for line in lines {
        let Ok(ip) = line else {panic!("Failed to read line, panic!")};
        if ip == "" {
            a = true;
        } else if a {
            let parsed_line: Vec<&str> = ip.split(" ").collect();
            let num_crates: usize = parsed_line[1].parse::<usize>().unwrap();
            let from_crate: usize = parsed_line[3].parse::<usize>().unwrap() - 1;
            let to_crate: usize = parsed_line[5].parse::<usize>().unwrap() - 1;
            for _ in 0..num_crates {
                let popped_value = crates[from_crate].pop_front().unwrap();
                crates[to_crate].push_front(popped_value);
            }
        } else {
            for x in 0..9 {
                let idx = 1 + ( 4 * x );
                let sel_char = ip.chars().nth(idx).unwrap();
                if char::is_alphabetic(sel_char) {
                    crates[x].push_back(sel_char);
                }
            }
        }
    }
    let first_letters = crates.into_iter().map(|col| col[0]).collect::<Vec<char>>();
    println!("part 1: {}", first_letters.iter().collect::<String>());
}


fn read_lines(file_path: &str) -> Lines<BufReader<File>>{
    let file = File::open(file_path.to_string()).unwrap();
    BufReader::new(file).lines()
}