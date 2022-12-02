pub fn day2_silver(input: &str) -> String {
    let res: u32 = input
        .lines()
        .map(|line| {
            let op = &line[0..1];
            let re = &line[2..3];
            match (op, re) {
                ("A", "X") => 3 + 1,
                ("B", "X") => 0 + 1,
                ("C", "X") => 6 + 1,
                ("A", "Y") => 6 + 2,
                ("B", "Y") => 3 + 2,
                ("C", "Y") => 0 + 2,
                ("A", "Z") => 0 + 3,
                ("B", "Z") => 6 + 3,
                ("C", "Z") => 3 + 3,
                _ => panic!(),
            }
        })
        .sum();
    res.to_string()
}

pub fn day2_gold(input: &str) -> String {
    let res: u32 = input
        .lines()
        .map(|line| {
            let op = &line[0..1];
            let re = &line[2..3];
            match (op, re) {
                ("A", "X") => 3 + 0,
                ("B", "X") => 1 + 0,
                ("C", "X") => 2 + 0,
                ("A", "Y") => 1 + 3,
                ("B", "Y") => 2 + 3,
                ("C", "Y") => 3 + 3,
                ("A", "Z") => 2 + 6,
                ("B", "Z") => 3 + 6,
                ("C", "Z") => 1 + 6,
                _ => panic!(),
            }
        })
        .sum();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn sample_silver() {
        let expected = "15".to_string();
        assert_eq!(day2_silver(INPUT), expected);
    }

    #[test]
    fn sample_gold() {
        let expected = "12".to_string();
        assert_eq!(day2_gold(INPUT), expected);
    }
}
