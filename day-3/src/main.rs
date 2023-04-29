use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        println!("{}", get_priorities_sum(lines));
    }
}

fn get_priorities_sum(lines: Lines<BufReader<File>>) -> i32 {
    let mut count = 0;
    for line in lines {
        let Ok(ip) = line else { todo!() };
        let mutual_length = ip.chars().count() / 2;
        let (left, right) = ip.split_at(mutual_length);
        let mut char_dict: HashMap<char, i8> = HashMap::new();
        for idx in 0..mutual_length {
            char_dict.insert(left.chars().nth(idx).unwrap(), 1);
        }
        for idx in 0..mutual_length {
            let sel_char = right.chars().nth(idx).unwrap();
            if char_dict.contains_key(&sel_char) {
                count += get_priority(sel_char);
                break;
            }
        }

    }
    count
}

fn get_priority(c: char) -> i32 {
    let mut char_val = c as i32;
    if c.is_lowercase() {
        char_val -= 96
    } else {
        char_val -= 38
    }
    char_val
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::with_capacity(1024, file).lines())
}
