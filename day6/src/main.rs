use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let sum_counts: usize = include_str!("puzzle_input.txt")
        .lines()
        .batching(read_group)
        .map(|group| group.len())
        .sum();

    println!("The sum of counts is {}.", sum_counts);
}

fn read_group<'a, T>(answers: &mut T) -> Option<HashSet<char>>
where
    T: Iterator<Item = &'a str> + Clone,
{
    let group: HashSet<char> = answers
        .take_while_ref(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .collect();
    answers.next();

    if group.is_empty() {
        None
    } else {
        Some(group)
    }
}
