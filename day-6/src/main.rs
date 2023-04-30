use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::string::ToString;
use std::collections::HashSet;


fn main() {
    let file_path = "./data/input.txt";
    let lines = read_lines(file_path);
    for line in lines {
        let Ok(ip) = line else { panic!("Failed to read line!")};
        let p1 = get_first_unique_window(ip.as_str(), 4);
        println!("part 1: {}", p1);
        let p2 = get_first_unique_window(ip.as_str(), 14);
        println!("part 2: {}", p2);
    }
}

fn get_first_unique_window(ip: &str, window_size: usize) -> usize {
    let mut result= 0;
    for idx in 0..(ip.len()-window_size-1) {
        let message: Vec<char> = ip.get(idx..idx+window_size).unwrap().chars().into_iter().collect();
        let charset: HashSet<char> = HashSet::from_iter(message);
        if charset.len() == window_size {
            result = idx+window_size;
            break;
        }
    }
    result
}

fn read_lines(file_path: &str) -> Lines<BufReader<File>>{
    let file = File::open(file_path.to_string()).unwrap();
    BufReader::new(file).lines()
}