use std::error::Error;
use scan_fmt::scan_fmt;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let rules = read_rules(include_str!("rules.txt"))?;
    let tickets = include_str!("tickets.txt");

    let error_rate = error_rate(&tickets, &rules);

    println!("Ticket scanning error rate = {}", error_rate);

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
        Ok(Rule { name, range1, range2 })
    }
}

fn read_rules(rules: &str) -> Result<Vec<Rule>, Box<dyn Error>> {
    rules.lines()
        .map(|rule| rule.parse())
        .collect()
}

fn error_rate(tickets: &str, rules: &[Rule]) -> u32 {
    tickets.lines()
        .flat_map(|line| line.split(','))
        .filter_map(|n| n.trim().parse::<u32>().ok())
        .filter(|&n| !rules.iter().any(|rule| rule.is_valid(n)))
        .sum()
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
        println!("{:#?}", rule);

        let expected = Rule {
            name: String::from("departure location"),
            range1: Range {
                min: 32,
                max: 842,
            },
            range2: Range {
                min: 854,
                max: 967,
            },
        };
        assert_eq!(rule.unwrap(), expected);
    }

    #[test]
    fn part1_example() {
        let rules = read_rules("class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50").unwrap();

        let tickets = "7,3,47
        40,4,50
        55,2,20
        38,6,12";

        assert_eq!(error_rate(&tickets, &rules), 71);
    }
}