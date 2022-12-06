use std::collections::HashSet;

pub fn is_unique(s: &str) -> bool {
    let mut uniq = HashSet::new();
    s.chars().all(move |x| uniq.insert(x))
}

pub fn day6_silver(input: &str) -> String {
    for i in 4..input.len() {
        if is_unique(&input[i - 4..i]) {
            return i.to_string();
        }
    }
    String::new()
}

pub fn day6_gold(input: &str) -> String {
    for i in 14..input.len() {
        if is_unique(&input[i - 14..i]) {
            return i.to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_silver() {
        assert_eq!(
            day6_silver("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "7".to_string()
        );
        assert_eq!(day6_silver("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5".to_string());
        assert_eq!(day6_silver("nppdvjthqldpwncqszvftbrmjlhg"), "6".to_string());
        assert_eq!(
            day6_silver("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "10".to_string()
        );
        assert_eq!(
            day6_silver("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "11".to_string()
        );
    }

    #[test]
    fn sample_gold() {
        assert_eq!(
            day6_gold("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "19".to_string()
        );
        assert_eq!(day6_gold("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23".to_string());
        assert_eq!(day6_gold("nppdvjthqldpwncqszvftbrmjlhg"), "23".to_string());
        assert_eq!(
            day6_gold("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "29".to_string()
        );
        assert_eq!(
            day6_gold("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "26".to_string()
        );
    }
}
