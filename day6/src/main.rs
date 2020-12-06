use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let groups: Vec<Vec<&str>> = include_str!("puzzle_input.txt")
        .lines()
        .batching(read_group)
        .collect();

    let part1: usize = groups.iter().map(|group| anyone_answered(group)).sum();

    println!("Part 1: The sum of counts is {}.", part1);

    let part2: usize = groups.iter().map(|group| everyone_answered(group)).sum();

    println!("Part 2: The sum of counts is {}.", part2);
}

fn read_group<'a, T>(answers: &mut T) -> Option<Vec<&'a str>>
where
    T: Iterator<Item = &'a str> + Clone,
{
    let group: Vec<_> = answers.take_while_ref(|line| !line.is_empty()).collect();
    answers.next();

    if group.is_empty() {
        None
    } else {
        Some(group)
    }
}

fn anyone_answered(group: &[&str]) -> usize {
    group
        .iter()
        .flat_map(|line| line.chars())
        .collect::<HashSet<char>>()
        .len()
}

fn everyone_answered(group: &[&str]) -> usize {
    let answers = group
        .iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .fold1(|a, b| a.intersection(&b).cloned().collect::<HashSet<char>>());
    
    match answers {
        Some(a) => a.len(),
        None => 0
    }
}