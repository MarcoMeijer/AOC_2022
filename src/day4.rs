pub fn day4_silver(input: &str) -> String {
    let x = input
        .lines()
        .map(|line| -> [[u32; 2]; 2] {
            line.split(",")
                .map(|ass| {
                    ass.split("-")
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .filter(|[[l1, r1], [l2, r2]]| (l1 <= l2 && r1 >= r2) || (l2 <= l1 && r2 >= r1))
        .collect::<Vec<_>>();
    x.len().to_string()
}

pub fn day4_gold(input: &str) -> String {
    let x = input
        .lines()
        .map(|line| -> [[u32; 2]; 2] {
            line.split(",")
                .map(|ass| {
                    ass.split("-")
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .filter(|[[l1, r1], [l2, r2]]| (l2 >= l1 && l2 <= r1) || (l1 >= l2 && l1 <= r2))
        .collect::<Vec<_>>();
    x.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn sample_silver() {
        let expected = "2".to_string();
        assert_eq!(day4_silver(INPUT), expected);
    }

    #[test]
    fn sample_gold() {
        let expected = "4".to_string();
        assert_eq!(day4_gold(INPUT), expected);
    }
}
