use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::ToString;
use std::vec::Vec;

struct Choice {
    score: i32,
    beats: String,
    beaten_by: String,
    draw: String,
}

fn main() {
    let file = File::open("./data/input.txt".to_string()).unwrap();
    let lines = BufReader::new(file).lines();

    let choices_map: HashMap<&str, Choice> = HashMap::from([
        ("A", Choice{ score: 1, beats: "C".to_string(), beaten_by: "B".to_string(), draw: "A".to_string()}),
        ("B", Choice{ score: 2, beats: "A".to_string(), beaten_by: "C".to_string(), draw: "B".to_string()}),
        ("C", Choice{ score: 3, beats: "B".to_string(), beaten_by: "A".to_string(), draw: "C".to_string()})
    ]);

    // part 1
    let part1_map: HashMap<&str, &str> = HashMap::from([
        ("X", "A"),
        ("Y", "B"),
        ("Z", "C")
    ]);

    let mut points = 0;
    for line in lines {
        let string = line.unwrap();
        let choices = string.split(" ");
        let vec: Vec<&str> = choices.collect();

        let our_choice = &choices_map[part1_map[vec[1]]];
        if our_choice.beats == vec[0] {
            // we win this round
            points += 6
        } else if our_choice.beaten_by == vec[0] {
            // we lose this round
            points += 0
        } else {
            // draw
            points += 3
        }
        points += our_choice.score;
    }

    println!("Answer to part 1: {}", points);

    // part 2
    let mut points = 0;
    for line in lines {
        let string = line.unwrap();
        let choices = string.split(" ");
        let vec: Vec<&str> = choices.collect();

        let mut our_choice: Choice;
        let their_choice = choices_map.get(vec[0]).unwrap();
        if vec[1] == "X" {
            // we need to win
            our_choice = choices_map.get(&(their_choice.beaten_by))
        } else if vec[1] == "Y" {
            // we need to draw
            our_choice = choices_map[&their_choice.get(draw)]
        } else {
            // we need to lose
            our_choice = choices_map[their_choice.get(beats)]
        }
        if our_choice.beats == vec[0] {
            // we win this round
            points += 6
        } else if our_choice.beaten_by == vec[0] {
            // we lose this round
            points += 0
        } else {
            // draw
            points += 3
        }
        points += our_choice.score;
    }
}