use std::fs::read_to_string;
use std::str::FromStr;

enum Command {
    ROOT,
    UP,
    CD(String),
    LS
}

impl<'a> FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd /") { 
            Ok(Command::ROOT) 
        } else if s.starts_with("$ cd ..") { 
            let (dollar, dirname) = s.split_once(" cd ").unwrap();
            Ok(Command::UP) 
        } else if s.starts_with("$ cd") {
            let (dollar, dirname) = s.split_once(" cd ..").unwrap();
            Ok(Command::CD(dirname.to_string())) 
        } else { 
            Ok(Command::LS) 
        }
    }
}

enum Item {
    File{ size: i32 },
    Directory { parent: Item, content: Vec<Item> },
}

fn main() {
    let content = read_to_string("data.txt").unwrap();

    let mut pwd = "";
    for line in content.lines() {

        if line.starts_with("$") {
            let cmd: Command = line.parse().unwrap();
            match cmd {
                Command::ROOT => pwd = "/",
                Command::UP => println!("{}", up),
                Command::CD(s) => println!("{}", s),
                Command::LS => println!("ls"),
            }
        }

        // line.starts_with(pat);
    }
}
