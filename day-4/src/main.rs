use std::io::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Range {
    start: i32,
    end: i32
}

impl Range {
    fn size(&self) -> i32 {
        self.end - self.start
    }
}

fn parse_range(input_str: &str) -> Range {
    let range_pair: Vec<&str> = input_str.split("-").collect();
    assert_eq!(range_pair.len(), 2);

    let start: i32 = range_pair[0].parse::<i32>().unwrap();
    let end: i32 = range_pair[1].parse::<i32>().unwrap();
    Range { start, end }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/input.txt") {
        let mut part1_count = 0;
        let mut part2_count = 0;

        for line in lines {
            let Ok(ip): Result<String, Error> = line else { panic!("failed to read line!") };
            let pair: Vec<&str> = ip.as_str().split(",").collect();
            assert_eq!(pair.len(), 2);

            let range_1: Range = parse_range(pair[0]);
            let range_2: Range = parse_range(pair[1]);

            // part 1
            let mut sorted: [Range; 2] = [range_1, range_2];
            sorted.sort_by_key(|a| a.size());
            let subset = &sorted[0];
            let superset = &sorted[1];
            if superset.start <= subset.start && superset.end >= subset.end {
                part1_count += 1;
            }

            // part 2
            sorted.sort_by_key(|a| a.end);
            let left = &sorted[0];
            let right = &sorted[1];
            if left.end >= right.start {
                part2_count += 1;
            }
        }
        println!("part 1: {}", part1_count);
        println!("part 2: {}", part2_count);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::with_capacity(1024, file).lines())
}
