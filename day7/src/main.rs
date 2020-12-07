use lazy_static::lazy_static;
use regex::Regex;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    colour: String,
    contents: HashMap<String, usize>,
}

fn main() {
    let rules: Vec<Rule> = include_str!("puzzle_input.txt")
        .lines()
        .map(parse_bag)
        .map(| (colour, contents) | Rule { colour, contents })
        .collect();

    let can_contain_gold = colours_containing("shiny gold", &rules);
    println!("{} bag colours can contain shiny gold:", can_contain_gold.len());
    for colour in can_contain_gold.iter() {
        println!("{}", colour);
    }
}

fn parse_bag(bag: &str) -> (String, HashMap<String, usize>) {
    let parts: Vec<_> = bag.split("bags contain").collect();

    let colour = parts[0].trim().to_owned();

    let contents = if parts[1].contains("no other") {
        HashMap::new()
    } else {
        parse_contents(&parts[1])
    };

    (colour, contents)
}

fn parse_contents(contents: &str) -> HashMap<String, usize> {
    lazy_static! {
        static ref SEPARATE_BAGS: Regex = Regex::new(r"bags?[.,]").unwrap();
    }

    SEPARATE_BAGS
        .split(contents)
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(|item| {
            let (count, colour) = scan_fmt!(item, r"{d} {[A-Za-z ]}", usize, String).unwrap();
            (colour, count)
        })
        .collect()
}

fn colours_containing(colour: &str, rules: &[Rule]) -> HashSet<String> {
    let mut colours = HashSet::new();

    for rule in rules {
        if rule.contents.contains_key(colour) {
            colours.insert(rule.colour.clone());
            let mut parents = colours_containing(&rule.colour, rules);
            for parent in parents.drain() {
                colours.insert(parent);
            }
        }
    }

    colours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bag() {
        let (colour, contents) = parse_bag("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(colour, "light red");
        assert_eq!(contents.len(), 2);
        assert_eq!(contents["bright white"], 1);
        assert_eq!(contents["muted yellow"], 2);
    }

    #[test]
    fn test_parse_bag_no_contents() {
        let (colour, contents) = parse_bag("faded blue bags contain no other bags.");
        assert_eq!(colour, "faded blue");
        assert!(contents.is_empty());
    }

    #[test]
    fn test_parse_bag_many_items() {
        let (colour, contents) = parse_bag("dotted magenta bags contain 1 mirrored maroon bag, 3 shiny red bags, 2 faded blue bags, 2 mirrored purple bags.");
        assert_eq!(colour, "dotted magenta");
        assert_eq!(contents.len(), 4);
        assert_eq!(contents["mirrored maroon"], 1);
        assert_eq!(contents["shiny red"], 3);
        assert_eq!(contents["faded blue"], 2);
        assert_eq!(contents["mirrored purple"], 2);
    }
}