use std::fs::read_to_string;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let content = read_to_string("data.txt").unwrap();

    let mut count = 0;
    for (_, line) in content.lines().enumerate() {
        let mut pairs: Vec<Pair> = line.split(",").map(|x| x.parse().unwrap()).collect();

        pairs.sort_by_key(|x| -x.size());

        let (bigger, smaller) = (pairs.get(0).unwrap(), pairs.get(1).unwrap());

        if bigger.left <= smaller.left && smaller.right <= bigger.right {
            count += 1;
        }
    }

    println!("{}", count);
}

#[derive(Debug)]
struct Pair {
    left: i32,
    right: i32,
}

impl Pair {
    fn size(&self) -> i32 {
        self.right - self.left
    }
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once("-").unwrap();
        let (l, r): (i32, i32) = (left.parse().unwrap(), right.parse().unwrap());

        Ok(Pair { left: l, right: r})
    }
}
