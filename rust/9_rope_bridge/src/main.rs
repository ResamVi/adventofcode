use std::fs::read_to_string;
use std::str::FromStr;
use std::collections::HashSet;
use anyhow::Result;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate (i32, i32);

#[derive(Debug)]
struct Coordinates(Vec<Coordinate>);

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let direction = match s {
            "U" => Coordinate(0, 1),
            "D" => Coordinate(0, -1),
            "L" => Coordinate(-1, 0),
            "R" => Coordinate(1, 0),
            _ => panic!("impossibru"),
        };

        Ok(direction)
    }
}

impl Coordinate {
    fn move_by(self, direction: Coordinate) -> Coordinate {
        Coordinate(self.0 + direction.0, self.1 + direction.1)
    }

    fn follow(self, target: Coordinate) -> Coordinate {
        let diff: Coordinate = Coordinate(target.0 - self.0, target.1 - self.1);

        // Stay if close
        if diff.close() {
            return self;
        }

        return match diff {
            Coordinate(_, 0) | Coordinate(0, _) => self.add(diff.unit()),
            Coordinate(_, _) => self.add(diff.cross()),
        };
    }

    fn close(&self) -> bool {
        self.0.pow(2) + self.1.pow(2) <= 1 || self.0.pow(2) == self.1.pow(2)
    }

    fn add(self, coord: Coordinate) -> Coordinate {
        Coordinate(self.0 + coord.0, self.1 + coord.1)
    }

    // horizontal/vertical moves (1 up, down, left, right)
    fn unit(self) -> Coordinate {
        Coordinate(self.0.signum(), self.1.signum())
    }

    // diagonal moves
    fn cross(self) -> Coordinate {
        return match self {
            Coordinate(_, 2) => Coordinate(self.0.signum(), 1),
            Coordinate(_, -2) => Coordinate(self.0.signum(), -1),
            Coordinate(2, _) => Coordinate(1, self.1.signum()),
            Coordinate(-2, _) => Coordinate(-1, self.1.signum()),
            Coordinate(_, _) => {
                println!("hmm: {:?}", self);
                panic!("wtf");
            },
        }
    }
}

impl IntoIterator for Coordinates {
    type Item = Coordinate;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Coordinates {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_once(" ").unwrap();

        let direction: Coordinate = left.parse()?;
        let count: usize = right.parse()?;

        let result = [direction].iter().cycle().cloned().take(count).collect();

        Ok(Coordinates(result))
    }
}

fn main() -> Result<()> {
    // let mut head = Coordinate(0, 0);
    // let mut tail = Coordinate(0, 0);

    // head = head.move_by(Coordinate(-1, -2));
    // let result = tail.follow(head);
    // println!("{:?}", result);

    let mut set: HashSet<Coordinate> = HashSet::new();

    let content = read_to_string("data.txt").unwrap();

    let mut head = Coordinate(0, 0);
    let mut tail = Coordinate(0, 0);

    for line in content.lines().into_iter() {
        let coords: Coordinates = line.parse()?;
        // println!("{:?}", coords);

        for coord in coords {
            head = head.move_by(coord);
            tail = tail.follow(head);
            // println!("{:?}", tail);
            set.insert(tail);
        }
    }
    println!("{}", set.len());


    return Ok(())
}
