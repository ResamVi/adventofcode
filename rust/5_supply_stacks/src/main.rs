use std::{fs::read_to_string, char};

fn main() {
    let content = read_to_string("data.txt").unwrap();

    let mut skipped = 0; // Amount of lines to get to the move instructions

    for (i, line) in content.lines().enumerate() {
        if line == "" {
            skipped = i+1;
            break;
        }

        // Put "|" so we can split crates
        let mut line: String = line
            .split("")
            .enumerate()
            .map(|(i, val)| if i % 4 == 0 { "|" } else { val } )
            .collect();

        // Remove first and last
        line.remove(0);
        line.pop();

        for cr in line.split("|").into_iter() {
            println!("-{}", cr);
        }
        println!("");
    }

    for line in content.lines().skip(skipped) {
        println!("{}", line);
    }


}
