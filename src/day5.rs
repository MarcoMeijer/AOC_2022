use nom::branch::alt;
use nom::character::complete::{alpha1, newline, not_line_ending};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::{bytes::streaming::tag, character::complete, IResult};

pub fn p_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, c) = alt((tag("   "), delimited(tag("["), alpha1, tag("]"))))(input)?;

    let result = match c {
        "   " => None,
        s => Some(s.as_bytes()[0] as char),
    };
    Ok((input, result))
}

pub fn p_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), p_crate)(input)
}

pub fn p_ship(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, lines) = separated_list1(newline, p_crate_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;

    let mut result = vec![vec![]; lines[0].len()];
    for line in lines.iter() {
        for (i, c) in line.iter().enumerate() {
            if let &Some(c) = c {
                result[i].push(c);
            }
        }
    }
    result.iter_mut().for_each(|x| x.reverse());
    Ok((input, result))
}

pub fn p_command(input: &str) -> IResult<&str, [usize; 3]> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((input, [amount as usize, from as usize - 1, to as usize - 1]))
}

pub fn day5_silver(input: &str) -> String {
    let (input, mut ship) = p_ship(input).unwrap();
    let (_, commands) = separated_list0(newline, p_command)(input).unwrap();

    for &[amount, from, to] in commands.iter() {
        let n = ship[from].len();
        let mut range = ship[from][n - amount..]
            .iter()
            .map(|x| *x)
            .collect::<Vec<_>>();
        range.reverse();
        ship[to].append(&mut range);
        ship[from].drain(n - amount..);
    }

    ship.iter().map(|x| x.last().unwrap()).collect()
}

pub fn day5_gold(input: &str) -> String {
    let (input, mut ship) = p_ship(input).unwrap();
    let (_, commands) = separated_list0(newline, p_command)(input).unwrap();

    for &[amount, from, to] in commands.iter() {
        let n = ship[from].len();
        let mut range = ship[from][n - amount..]
            .iter()
            .map(|x| *x)
            .collect::<Vec<_>>();
        ship[to].append(&mut range);
        ship[from].drain(n - amount..);
    }

    ship.iter().map(|x| x.last().unwrap()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn sample_silver() {
        let expected = "CMZ".to_string();
        assert_eq!(day5_silver(INPUT), expected);
    }

    #[test]
    fn sample_gold() {
        let expected = "MCD".to_string();
        assert_eq!(day5_gold(INPUT), expected);
    }
}
