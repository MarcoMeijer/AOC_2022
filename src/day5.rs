use nom::branch::alt;
use nom::character::complete::{newline, not_line_ending};
use nom::multi::{separated_list0, separated_list1};
use nom::{bytes::streaming::tag, character::complete, character::complete::anychar, IResult};

pub fn p_crate(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("[")(input)?;
    let (input, c) = anychar(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, c))
}

pub fn p_empty_crate(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, ' '))
}

pub fn p_option_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, c) = alt((p_crate, p_empty_crate))(input)?;
    Ok((input, if c == ' ' { None } else { Some(c) }))
}

pub fn p_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), p_option_crate)(input)
}

pub fn p_ship(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, lines) = separated_list1(newline, p_crate_line)(input)?;
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

pub fn day5_silver(input: &str) -> IResult<&str, String> {
    let (input, mut ship) = p_ship(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, commands) = separated_list0(newline, p_command)(input)?;

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

    let result = ship.iter().map(|x| x.last().unwrap()).collect::<String>();
    Ok((input, result))
}

pub fn day5_gold(input: &str) -> IResult<&str, String> {
    let (input, mut ship) = p_ship(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, commands) = separated_list0(newline, p_command)(input)?;

    for &[amount, from, to] in commands.iter() {
        let n = ship[from].len();
        let mut range = ship[from][n - amount..]
            .iter()
            .map(|x| *x)
            .collect::<Vec<_>>();
        ship[to].append(&mut range);
        ship[from].drain(n - amount..);
    }

    let result = ship.iter().map(|x| x.last().unwrap()).collect::<String>();
    Ok((input, result))
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
    fn sample_silver() -> Result<(), ()> {
        let expected = "CMZ".to_string();
        let (_, result) = day5_silver(INPUT).map_err(|_| ())?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn sample_gold() -> Result<(), ()> {
        let expected = "MCD".to_string();
        let (_, result) = day5_gold(INPUT).map_err(|_| ())?;
        assert_eq!(result, expected);
        Ok(())
    }
}
