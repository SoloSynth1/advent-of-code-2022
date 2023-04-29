use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/input.txt") {
        let mut part1_count = 0;
        let mut part2_count = 0;
        let mut part2_buf: Vec<String> = Vec::new();

        for (idx, line) in lines.enumerate() {
            // part 1
            let Ok(ip) = line else { todo!() };
            let (left, right) = ip.split_at(ip.chars().count() / 2);
            let mutual_char = get_mutual_characters(left, right);
            assert_eq!(mutual_char.len(), 1);
            part1_count += get_priority(mutual_char.chars().nth(0).unwrap());

            // part 2
            part2_buf.push(ip);
            if idx % 3 == 2 {
                // calculate mutual chars
                let mutual_char = get_mutual_characters(&get_mutual_characters(&part2_buf[0], &part2_buf[1]), &part2_buf[2]);
                assert_eq!(mutual_char.len(), 1);
                part2_count += get_priority(mutual_char.chars().nth(0).unwrap());
                // clear buffer
                part2_buf = Vec::new();
            }
        }
        println!("part 1: {}", part1_count);
        println!("part 2: {}", part2_count);
    }
}

fn get_mutual_characters(a: &str, b: &str) -> String {
    let mut results: Vec<char> = Vec::new();
    let mut a_chars = HashSet::new();
    for idx in 0..a.len() {
        a_chars.insert(a.chars().nth(idx).unwrap());
    }
    for idx in 0..b.len() {
        let b_char = b.chars().nth(idx).unwrap();
        if a_chars.contains(&b_char) && !results.contains(&b_char) {
            results.push(b_char);
        }
    }
    let result: String = results.iter().collect();
    result
}

fn get_priority(c: char) -> i32 {
    let mut char_val = c as i32;
    char_val += if c.is_lowercase() {-96} else {-38};   // convert ascii values to AoC rules
    char_val
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::with_capacity(1024, file).lines())
}
