use crate::matching::*;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Rule {
    Sequence(Vec<usize>),
    Either((Vec<usize>, Vec<usize>)),
    Literal(char),
}

pub fn parse_rules(input: &str) -> HashMap<usize, Rule> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        if let Ok((index, character)) = scan_fmt!(line, "{d}: \"{}\"", usize, char) {
            rules.insert(index, Rule::Literal(character));
        } else if let Ok((index, a, b)) =
            scan_fmt!(line, "{d}: {[0-9 ]} | {[0-9 ]}", usize, String, String)
        {
            rules.insert(
                index,
                Rule::Either((parse_sequence(&a), parse_sequence(&b))),
            );
        } else if let Ok((index, numbers)) = scan_fmt!(line, "{d}: {[0-9 ]}", usize, String) {
            rules.insert(index, Rule::Sequence(parse_sequence(&numbers)));
        }
    }

    rules
}

fn parse_sequence(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

pub fn create_matcher(index: usize, rules: &HashMap<usize, Rule>) -> BoxedMatcher {
    let rule = rules.get(&index).expect("Looking for non-existent rule");
    match rule {
        Rule::Literal(c) => BoxedMatcher::new(literal(*c)),
        Rule::Sequence(numbers) => {
            let mut matcher = create_matcher(numbers[0], rules);
            for n in &numbers[1..] {
                matcher = BoxedMatcher::new(pair(matcher, create_matcher(*n, rules)));
            }
            matcher
        }
        Rule::Either((a, b)) => {
            let mut left = create_matcher(a[0], rules);
            for n in &a[1..] {
                left = BoxedMatcher::new(pair(left, create_matcher(*n, rules)));
            }

            let mut right = create_matcher(b[0], rules);
            for n in &b[1..] {
                right = BoxedMatcher::new(pair(right, create_matcher(*n, rules)));
            }

            BoxedMatcher::new(either(left, right))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let rules = parse_rules(
            "0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"",
        );

        let matcher = create_matcher(0, &rules);

        assert!(matcher.matches("ababbb").is_ok());
        assert_eq!(true, matcher.exactly_matches("ababbb"));
        assert!(matcher.matches("abbbab").is_ok());
        assert!(matcher.matches("bababa").is_err());
        assert!(matcher.matches("aaabbb").is_err());
        assert_eq!(Ok("b"), matcher.matches("aaaabbb"));
        assert_eq!(false, matcher.exactly_matches("aaaabbb"));
    }
}
