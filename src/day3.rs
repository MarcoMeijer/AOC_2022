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
            panic!();
        })
        .map(|c| match c {
            'a'..='z' => (c as u8 - 96) as u32,
            'A'..='Z' => (c as u8 - 38) as u32,
            _ => 0,
        })
        .sum();
    res.to_string()
}

pub fn day3_gold(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let total: u32 = (0..lines.len() / 3)
        .map(|i| [lines[i * 3], lines[i * 3 + 1], lines[i * 3 + 2]])
        .map(|group| {
            for i in 0..group[0].len() {
                let pat = &group[0][i..i + 1];
                if group[1].contains(pat) && group[2].contains(pat) {
                    return group[0].as_bytes()[i] as char;
                }
            }
            panic!();
        })
        .map(|c| match c {
            'a'..='z' => (c as u8 - 96) as u32,
            'A'..='Z' => (c as u8 - 38) as u32,
            _ => 0,
        })
        .sum();
    total.to_string()
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

    #[test]
    fn sample_gold() {
        let expected = "70".to_string();
        assert_eq!(day3_gold(INPUT), expected);
    }
}
