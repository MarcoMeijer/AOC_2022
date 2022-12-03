pub fn day3_silver(input: &str) -> String {
    let res: u32 = input
        .lines()
        .map(|line| {
            let n = line.len() / 2;
            let l = &line[..n];
            let r = &line[n..];
            for i in 0..n {
                if l.contains(&r[i..i + 1]) {
                    return r.as_bytes()[i] as char;
                }
            }
            l.as_bytes()[0] as char
        })
        .map(|c| match c {
            'a'..='z' => (c as u8 - 96) as u32,
            'A'..='Z' => (c as u8 - 38) as u32,
            _ => 0,
        })
        .sum();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn sample_silver() {
        let expected = "157".to_string();
        assert_eq!(day3_silver(INPUT), expected);
    }
}
