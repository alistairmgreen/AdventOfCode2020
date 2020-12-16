use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let rules = read_rules(include_str!("rules.txt"))?;
    let tickets = include_str!("tickets.txt");

    let error_rate = error_rate(&tickets, &rules);

    println!("Ticket scanning error rate = {}", error_rate);

    let tickets = valid_tickets(tickets, &rules);
    let fields = field_positions(&tickets, &rules);

    let my_ticket = vec![
        139, 67, 71, 59, 149, 89, 101, 83, 107, 103, 79, 157, 151, 113, 61, 109, 73, 97, 137, 53,
    ];

    let mut product: u64 = 1;
    for (name, &position) in &fields {
        if name.starts_with("departure") {
            println!(
                "{} is at position {}, value = {}.",
                name, position, my_ticket[position]
            );
            product *= my_ticket[position];
        }
    }

    println!("Product = {}", product);

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    pub fn contains(&self, value: u32) -> bool {
        value >= self.min && value <= self.max
    }
}

impl FromStr for Range {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = scan_fmt!(s, "{d}-{d}", u32, u32)?;
        Ok(Range { min, max })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Rule {
    name: String,
    range1: Range,
    range2: Range,
}

impl Rule {
    fn is_valid(&self, value: u32) -> bool {
        self.range1.contains(value) || self.range2.contains(value)
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, range1, range2) = scan_fmt!(s, "{[a-z ]}: {} or {}", String, String, String)?;
        let range1 = range1.parse()?;
        let range2 = range2.parse()?;
        Ok(Rule {
            name,
            range1,
            range2,
        })
    }
}

fn read_rules(rules: &str) -> Result<Vec<Rule>, Box<dyn Error>> {
    rules.lines().map(|rule| rule.parse()).collect()
}

fn error_rate(tickets: &str, rules: &[Rule]) -> u32 {
    tickets
        .lines()
        .flat_map(|line| line.split(','))
        .filter_map(|n| n.trim().parse::<u32>().ok())
        .filter(|&n| !rules.iter().any(|rule| rule.is_valid(n)))
        .sum()
}

fn valid_tickets(tickets: &str, rules: &[Rule]) -> Vec<Vec<u32>> {
    tickets
        .lines()
        .filter_map(|line| {
            line.split(',')
                .map(|n| n.trim().parse())
                .collect::<Result<Vec<u32>, ParseIntError>>()
                .ok()
        })
        .filter(|numbers| {
            numbers
                .iter()
                .all(|&n| rules.iter().any(|rule| rule.is_valid(n)))
        })
        .collect()
}

fn field_positions(tickets: &[Vec<u32>], rules: &[Rule]) -> HashMap<String, usize> {
    let mut possibilities = HashMap::with_capacity(rules.len());
    for rule in rules {
        possibilities.insert(
            rule.name.clone(),
            (0..rules.len()).collect::<HashSet<usize>>(),
        );
    }

    for ticket in tickets {
        for (index, &value) in ticket.iter().enumerate() {
            for rule in rules {
                if !rule.is_valid(value) {
                    possibilities.get_mut(&rule.name).unwrap().remove(&index);
                }
            }
        }
    }

    while possibilities.values().any(|set| set.len() > 1) {
        let unique: HashSet<usize> = possibilities
            .values()
            .filter(|set| set.len() == 1)
            .map(|set| set.iter().cloned().next().unwrap())
            .collect();

        assert!(!unique.is_empty());

        for possibility in possibilities.values_mut() {
            if possibility.len() > 1 {
                possibility.retain(|n| !unique.contains(n));
            }
        }
    }

    possibilities
        .into_iter()
        .map(|(name, values)| (name, values.into_iter().next().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_range() {
        let range: Range = "32-842".parse().unwrap();
        assert_eq!(range, Range { min: 32, max: 842 });
    }

    #[test]
    fn test_parse_rule() {
        let rule: Result<Rule, Box<dyn Error>> = "departure location: 32-842 or 854-967".parse();

        let expected = Rule {
            name: String::from("departure location"),
            range1: Range { min: 32, max: 842 },
            range2: Range { min: 854, max: 967 },
        };
        assert_eq!(rule.unwrap(), expected);
    }

    #[test]
    fn part1_example() {
        let rules = read_rules(
            "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50",
        )
        .unwrap();

        let tickets = "7,3,47
        40,4,50
        55,2,20
        38,6,12";

        assert_eq!(error_rate(&tickets, &rules), 71);
    }

    #[test]
    fn test_valid_tickets() {
        let rules = read_rules(
            "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50",
        )
        .unwrap();

        let tickets = "7,3,47
        40,4,50
        55,2,20
        38,6,12";

        let valid = valid_tickets(&tickets, &rules);
        assert_eq!(valid, vec![vec![7, 3, 47]]);
    }

    #[test]
    fn part2_example() {
        let rules = read_rules(
            "class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19",
        )
        .unwrap();

        let tickets = valid_tickets(
            "3,9,18
        15,1,5
        5,14,9",
            &rules,
        );

        let positions = field_positions(&tickets, &rules);
        assert_eq!(positions["row"], 0);
        assert_eq!(positions["class"], 1);
        assert_eq!(positions["seat"], 2);
    }
}
