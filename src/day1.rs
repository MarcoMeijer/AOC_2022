pub fn day1(input: &str) -> String {
    let mut elves: Vec<_> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<u64>().unwrap()).sum::<u64>())
        .collect();
    elves.sort();
    elves.reverse();
    let last_three: u64 = (&elves[0..3]).iter().sum();
    last_three.to_string()
}
