fn main() {
    let lines = include_str!("data.txt")
        .lines()
		.map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let mut groups = lines.split(|line| line.is_none())
        .into_iter()
        .map(|group| {
            group
                .iter()
				.map(|x| x.unwrap())
				.sum::<u64>()
        })
        .collect::<Vec<_>>();

	groups.sort();
	groups.reverse();

	let result: u64 = groups.iter().take(3).sum();

    println!("{result:?}");
}
