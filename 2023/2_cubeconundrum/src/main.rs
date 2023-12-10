use std::{fs, str::FromStr};
use regex::Regex;

struct Game {
    id: i8,
    subsets: Vec<Subset>
}

struct Subset(i8, i8, i8);

enum Color {
    Red,
    Green,
    Blue
}

// impl FromStr for Game {
//     type Err;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }


fn main() {
    // let contents = fs::read_to_string("file.txt").expect("to have a file");
    //
    // for line in contents.split("\n").into_iter() {
    //     println!("Content: {line}");
    // }

    let re = Regex::new(r"Game ([0-9]+): (.+)$").unwrap();
    let hay = "Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green; 1 blue, 3 green, 7 red; 5 red, 3 green, 1 blue";

    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(hay).map(|c| c.extract()) {
        results.push((path, lineno, line));
    }

    println!("{:?}", results);
}
