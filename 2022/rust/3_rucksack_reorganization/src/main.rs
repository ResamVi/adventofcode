use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let content = read_to_string("data.txt").unwrap();

    let result: i32 = content
        .lines()
        .map(|x|  {
            let (left, right) = x.split_at(x.len()/2);

            let mut set: HashSet<char> = HashSet::new();
            set.extend(left.chars());

            for (_, ch) in right.chars().enumerate() {
                if !set.contains(&ch) {
                    continue
                }
                    
                let result = if ch.is_uppercase() { ch as u8 - 38 } else { ch as u8 - 96 };
                return result as i32;
            }

            return 0;
        })
        .sum();
    

    println!("{}", result);
}
