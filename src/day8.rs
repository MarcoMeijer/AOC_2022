pub fn day8_silver(input: &str) -> String {
    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let n = trees.len();
    let m = trees[0].len();

    let res: u32 = (0..n)
        .flat_map(|i| (0..m).map(move |j| (i, j)))
        .map(|(x, y)| {
            let sz = trees[x][y];
            if x == 0
                || y == 0
                || x == n - 1
                || y == m - 1
                || trees[x][0..y].iter().max().unwrap() < &sz
                || trees[x][y + 1..m].iter().max().unwrap() < &sz
                || trees[0..x].iter().map(|a| a[y]).max().unwrap() < sz
                || trees[x + 1..n].iter().map(|a| a[y]).max().unwrap() < sz
            {
                1
            } else {
                0
            }
        })
        .sum();
    res.to_string()
}

pub fn get_len(values: Vec<u128>, mx: u128) -> u128 {
    let mut i = 0;
    for &x in values.iter() {
        i += 1;
        if x >= mx {
            return i;
        }
    }
    i
}

pub fn day8_gold(input: &str) -> String {
    let trees: Vec<Vec<u128>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u128)
                .collect()
        })
        .collect();

    let n = trees.len();
    let m = trees[0].len();

    let res = (0..n)
        .flat_map(|i| (0..m).map(move |j| (i, j)))
        .map(|(x, y)| {
            let sz = trees[x][y];
            let left = if y == 0 {
                0
            } else {
                get_len(trees[x][0..y].iter().map(|&x| x).rev().collect(), sz)
            };
            let right = if y == m - 1 {
                0
            } else {
                get_len(trees[x][y + 1..m].iter().map(|&x| x).collect(), sz)
            };
            let up = if x == 0 {
                0
            } else {
                get_len(trees[0..x].iter().map(|x| x[y]).rev().collect(), sz)
            };
            let down = if x == n - 1 {
                0
            } else {
                get_len(trees[x + 1..n].iter().map(|x| x[y]).collect(), sz)
            };
            left * right * up * down
        })
        .max()
        .unwrap();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn sample_silver() {
        let expected = "21".to_string();
        assert_eq!(day8_silver(INPUT), expected);
    }

    #[test]
    fn sample_gold() {
        let expected = "8".to_string();
        assert_eq!(day8_gold(INPUT), expected);
    }
}
