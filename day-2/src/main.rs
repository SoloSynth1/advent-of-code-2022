use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::string::ToString;
use std::vec::Vec;


fn main() {
    let relation: HashMap<char, char> = HashMap::from([
        ('r', 's'), ('s', 'p'), ('p', 'r')
    ]);

    let file_path = "./data/input.txt";
    let lines = read_lines(file_path);

    let mut part1_score = 0;
    let mut part2_score = 0;

    for line in lines {
        let Ok(ip) = line else {panic!("Failed to read line, panic!")};
        let pair: Vec<&str> = ip.split(" ").collect();
        assert_eq!(pair.len(), 2);

        // part 1
        let oppo_choice: char = match pair[0] {
            "A" => 'r',
            "B" => 'p',
            "C" => 's',
            &_ => panic!("Not implemented!"),
        };

        let your_choice_p1: char = match pair[1] {
            "X" => 'r',
            "Y" => 'p',
            "Z" => 's',
            &_ => panic!("Not implemented!"),
        };

        let match_scores: HashMap<char, i32> = HashMap::from([('w', 6), ('d', 3), ('l', 0)]);
        let choice_scores: HashMap<char, i32> = HashMap::from([('r', 1), ('p', 2), ('s', 3)]);

        if relation[&your_choice_p1] == oppo_choice {
            // you beat the opponent
            part1_score += match_scores[&'w'];
        } else if relation[&oppo_choice] == your_choice_p1 {
            // opponent beats you
            part1_score += match_scores[&'l']
        } else {
            // draw
            part1_score += match_scores[&'d']
        }
        part1_score += choice_scores[&your_choice_p1];

        // part 2
        let your_choice_p2: char = match pair[1] {
            "X" => 'l',
            "Y" => 'd',
            "Z" => 'w',
            &_ => panic!("Not implemented!"),
        };

        part2_score += match your_choice_p2 {
            'l' => choice_scores[&relation[&oppo_choice]],
            'd' => choice_scores[&oppo_choice],
            'w' => choice_scores[&relation[&relation[&oppo_choice]]],
            _ => panic!("Not implemented!"),
        };

        part2_score += match_scores[&your_choice_p2];

    }
    println!("part 1: {}", part1_score);
    println!("part 2: {}", part2_score);
}

fn read_lines(file_path: &str) -> Lines<BufReader<File>>{
    let file = File::open(file_path.to_string()).unwrap();
    BufReader::new(file).lines()
}