use std::{fs, str::FromStr};
use regex::Regex;
use std::string::ParseError;
use lazy_static::lazy_static;

lazy_static! {
    static ref GAME: Regex = Regex::new(r"Game ([0-9]+): (.+)$").unwrap();

    static ref RED: Regex = Regex::new(r"([0-9]+) red").unwrap();
    static ref GREEN: Regex = Regex::new(r"([0-9]+) green").unwrap();
    static ref BLUE: Regex = Regex::new(r"([0-9]+) blue").unwrap();
}

#[derive(std::fmt::Debug)]
struct Game {
    id: i32,
    subsets: Vec<Subset>
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = GAME.captures(s).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap_or(0);
        let subsets = GAME.captures(s).unwrap().get(2).unwrap().as_str();

        let mut result: Vec<Subset> = vec!();
        for subset in subsets.split("; ").into_iter() {
            result.push(subset.parse::<Subset>().unwrap());
        }

        Ok(Game { id, subsets: result })
    }
}


#[derive(std::fmt::Debug, Clone)]
struct Subset(i32, i32, i32);

impl FromStr for Subset {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let red_count = match RED.captures(s) {
            Some(cap) => cap.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0),
            None => 0,
        };

        let green_count = match GREEN.captures(s) {
            Some(cap) => cap.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0),
            None => 0,
        };

        let blue_count = match BLUE.captures(s) {
            Some(cap) => cap.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0),
            None => 0,
        };

        return Ok(Subset(red_count, green_count, blue_count))
    }
}

// Exercise 1
fn is_possible(game: &Game) -> bool {
    for subset in &game.subsets {
        if subset.0 > 12 || subset.1 > 13 || subset.2 > 14 {
            return false
        }
    }
    return true;
}

// Exercise 2
fn power(game: &Game) -> i32 {
    let mut reds: Vec<i32> = vec!();
    let mut greens: Vec<i32> = vec!();
    let mut blues: Vec<i32> = vec!();

    for Subset(r, g, b) in game.subsets.clone().into_iter() {
        reds.push(r);
        greens.push(g);
        blues.push(b);
    }

    let red = reds.iter().filter(|&&v| v != 0).max().unwrap();
    let green = greens.iter().filter(|&&v| v != 0).max().unwrap();
    let blue = blues.iter().filter(|&&v| v != 0).max().unwrap();

    return *red * *green * *blue;
}

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();

    let mut sum = 0;
    for line in file.lines() {
        let game = line.parse::<Game>().unwrap();
        sum += power(&game);
    }

    println!("{sum}");
}
