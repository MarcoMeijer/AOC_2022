use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{self, newline, not_line_ending, space1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Item {
    Directory(String),
    File(u64, String),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<Item>),
}

fn p_directory(input: &str) -> IResult<&str, Item> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = not_line_ending(input)?;
    Ok((input, Item::Directory(name.to_string())))
}

fn p_file(input: &str) -> IResult<&str, Item> {
    let (input, size) = complete::u64(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = not_line_ending(input)?;
    Ok((input, Item::File(size, name.to_string())))
}

fn p_item(input: &str) -> IResult<&str, Item> {
    alt((p_directory, p_file))(input)
}

fn p_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = not_line_ending(input)?;
    Ok((input, Command::Cd(dir.to_string())))
}

fn p_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;

    let (input, items) = separated_list1(newline, p_item)(input)?;

    Ok((input, Command::Ls(items)))
}

fn p_command(input: &str) -> IResult<&str, Command> {
    alt((p_cd, p_ls))(input)
}

pub fn p_directory_sizes(input: &str) -> IResult<&str, HashMap<String, u64>> {
    let (input, commands) = separated_list1(newline, p_command)(input)?;

    let mut stack = vec!["/".to_string()];
    let mut dirs = HashMap::new();

    for command in commands.iter() {
        let cur_dir = stack.last().unwrap().clone();
        match command {
            Command::Cd(dir) => match dir.as_str() {
                ".." => {
                    stack.pop();
                }
                "/" => stack = vec!["/".to_string()],
                _ => {
                    stack.push(format!("{cur_dir}{dir}/").to_string());
                }
            },
            Command::Ls(items) => {
                dirs.insert(cur_dir, items);
            }
        };
    }

    let mut dirs: Vec<_> = dirs.into_iter().collect();
    dirs.sort_by(|x, y| y.0.len().cmp(&x.0.len()));
    let mut dir_size = HashMap::new();
    for (k, items) in dirs.iter() {
        let cur_dir = k.clone();
        let size: u64 = items
            .iter()
            .map(|item| match item {
                Item::Directory(name) => dir_size
                    .get(&format!("{cur_dir}{name}/").to_string())
                    .unwrap(),
                Item::File(size, _) => size,
            })
            .sum();
        dir_size.insert(cur_dir, size);
    }

    Ok((input, dir_size))
}

pub fn day7_silver(input: &str) -> String {
    let (_, dir_size) = p_directory_sizes(input).unwrap();

    let total: u64 = dir_size
        .into_iter()
        .map(|x| x.1)
        .filter(|x| *x < 100000)
        .sum();

    total.to_string()
}

pub fn day7_gold(input: &str) -> String {
    let (_, dir_size) = p_directory_sizes(input).unwrap();

    let needed: u64 = dir_size[&"/".to_string()] - 40000000;
    let mut solutions: Vec<u64> = dir_size
        .into_iter()
        .map(|x| x.1)
        .filter(|x| *x >= needed)
        .collect();
    solutions.sort();

    solutions[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn sample_silver() {
        assert_eq!(day7_silver(INPUT), "95437".to_string());
    }

    #[test]
    fn sample_gold() {
        assert_eq!(day7_gold(INPUT), "24933642".to_string());
    }
}
