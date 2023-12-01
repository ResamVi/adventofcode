use std::{fs::read_to_string, collections::HashSet};

use itertools::Itertools;

fn main() {
    let content = read_to_string("data.txt").unwrap();
    
    let mut set: HashSet<char> = HashSet::new();
    let mut index = 4;
    for (a, b, c, d) in content.chars().tuple_windows() {
        set.insert(a);
        set.insert(b);
        set.insert(c);
        set.insert(d);

        if set.len() == 4 {
            break;
        }
        set.clear();

        index+=1;
    }

    println!("{}", index);

}

