use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
	let mut file = File::open("data.txt")?;
	let mut contents = String::new();

    file.read_to_string(&mut contents);

    let mut elves: HashMap<i32, i32> = HashMap::new();

    let mut i = 0;

    for x in contents.split("\n") {
        if x == "" {
            i += 1;
            continue;
        }

        elves.entry(i)
            .and_modify(|curr| *curr += x.parse::<i32>().unwrap())
            .or_insert(x.parse().unwrap());
    }

    let mut max = 0;
    for x in elves.into_iter() {
        if x.1 > max {
            max = x.1;
        }
    }
    println!("{}", max);

	Ok(())
}
