use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;

fn main() {
    let mut f = File::open("data.txt").unwrap();

            // selection := map[string]int{
            //     "X": 1,
            //     "Y": 2,
            //     "Z": 3,
        // }

        // outcome := map[string]int{
            //     "A X": 3,
            //     "C X": 6,
            //     "A Y": 6,
            //     "B Y": 3,
            //     "B Z": 6,
            //     "C Z": 3,
        // }

    // let selection: HashMap<String, i32> = HashMap::from([
    //     ("X".to_string(), 1),
    //     ("Y".to_string(), 2),
    //     ("Z".to_string(), 3),
    // ]);

    let outcome: HashMap<String, i32> = HashMap::from([
        ("A X".to_string(), 3),
        ("C X".to_string(), 6),
        ("A Y".to_string(), 6),
        ("B Y".to_string(), 3),
        ("B Z".to_string(), 6),
        ("C Z".to_string(), 3),
    ]);

    let mut total = 0;
    for x in BufReader::new(f).lines() {
        let line = x.unwrap();
        let (opponent, response) = line.split_once(" ").unwrap();

        total += score(opponent, response);
    }

    println!("{}", total);
}

fn score(opponent: &str, response: &str) -> i32 {
    let mut score = match (opponent, response) {
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("C", "X") | ("A", "Y") | ("B", "Z") => 6,
        (_, _) => 0,
    };

    score += match response {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    score
}
