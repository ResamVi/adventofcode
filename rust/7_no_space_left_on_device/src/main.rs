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
        if s == "$ cd /" { 
            Ok(Command::ROOT) 
        } else if s == "$ cd .." { 
            Ok(Command::UP) 
        } else if s.starts_with("$ cd") {
            let (dollar, dirname) = s.split_once(" cd ").unwrap();
            Ok(Command::CD(dirname.to_string())) 
        } else { 
            Ok(Command::LS) 
        }
    }
}

struct File {
    name: String,
    size: i32,
    parent: Option<Box<File>>,
    content: Vec<File>,
}

impl File {
    fn create_file(self) {

    }

    // Only for directories
    fn createDirectory(self) {

    }
}

#[derive(Debug)]
struct Path(String);

impl Path {
    fn up(&self) {
        println!("{}", self.0);
    }
    fn to(&mut self, to: &str) {
        self.0.push_str(to);
        self.0.push_str("/");
    }
}

fn main() {
    let content = read_to_string("data.txt").unwrap();

    let root: File = File { 
        name: "/".to_string(),
        size: 0,
        parent: Option::None,
        content: Vec::new(),
    };

    let current: File = root;

    let mut pwd = Path("".to_string());
    for line in content.lines() {

        if line.starts_with("$") {
            let cmd: Command = line.parse().unwrap();
            match cmd {
                Command::ROOT => { 
                    pwd = Path("/".to_string());
                }
                Command::UP => { 

                }
                Command::CD(s) => {
                    // println!("{}", s);
                    pwd.to(&s);
                    println!("{:?}", pwd);
                }
                Command::LS => println!("ls"),
            }
        }

        // line.starts_with(pat);
    }
}
