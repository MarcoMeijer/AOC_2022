use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn p_line(input: &str) -> IResult<&str, ((i32, i32), u32)> {
    let (input, (d, steps)) = separated_pair(anychar, tag(" "), complete::u32)(input)?;
    Ok((
        input,
        (
            match d {
                'U' => (0, 1),
                'D' => (0, -1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => (0, 0),
            },
            steps,
        ),
    ))
}

pub fn day9_silver(input: &str) -> String {
    let lines = separated_list1(newline, p_line)(input).unwrap().1;
    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);
    let mut tail_positions = HashSet::new();
    tail_positions.insert(t_pos);
    for (delta, steps) in lines {
        for _ in 0..steps {
            h_pos = (h_pos.0 + delta.0, h_pos.1 + delta.1);
            if (h_pos.0 - t_pos.0).abs() >= 1
                && (h_pos.1 - t_pos.1).abs() >= 1
                && (h_pos.0 - t_pos.0).abs() + (h_pos.1 - t_pos.1).abs() >= 3
            {
                t_pos.0 += if t_pos.0 < h_pos.0 { 1 } else { -1 };
                t_pos.1 += if t_pos.1 < h_pos.1 { 1 } else { -1 };
            } else {
                if (h_pos.0 - t_pos.0).abs() >= 2 {
                    t_pos.0 += if t_pos.0 < h_pos.0 { 1 } else { -1 };
                }
                if (h_pos.1 - t_pos.1).abs() >= 2 {
                    t_pos.1 += if t_pos.1 < h_pos.1 { 1 } else { -1 };
                }
            }
            tail_positions.insert(t_pos);
        }
    }
    tail_positions.len().to_string()
}

pub fn day9_gold(input: &str) -> String {
    let lines = separated_list1(newline, p_line)(input).unwrap().1;
    let mut rope = vec![(0, 0); 10];
    let mut tail_positions = HashSet::new();
    tail_positions.insert(*rope.last().unwrap());
    for (delta, steps) in lines {
        for _ in 0..steps {
            rope[0] = (rope[0].0 + delta.0, rope[0].1 + delta.1);
            for j in 1..10 {
                if (rope[j - 1].0 - rope[j].0).abs() >= 1
                    && (rope[j - 1].1 - rope[j].1).abs() >= 1
                    && (rope[j - 1].0 - rope[j].0).abs() + (rope[j - 1].1 - rope[j].1).abs() >= 3
                {
                    rope[j].0 += if rope[j].0 < rope[j - 1].0 { 1 } else { -1 };
                    rope[j].1 += if rope[j].1 < rope[j - 1].1 { 1 } else { -1 };
                } else {
                    if (rope[j - 1].0 - rope[j].0).abs() >= 2 {
                        rope[j].0 += if rope[j].0 < rope[j - 1].0 { 1 } else { -1 };
                    }
                    if (rope[j - 1].1 - rope[j].1).abs() >= 2 {
                        rope[j].1 += if rope[j].1 < rope[j - 1].1 { 1 } else { -1 };
                    }
                }
            }
            tail_positions.insert(*rope.last().unwrap());
        }
    }
    tail_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn sample_silver() {
        assert_eq!(day9_silver(INPUT), "13".to_string());
    }

    #[test]
    fn sample_gold() {
        assert_eq!(day9_gold(INPUT), "1".to_string());
        assert_eq!(
            day9_gold(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            "36".to_string()
        )
    }
}
