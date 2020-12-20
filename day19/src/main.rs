use day19::parsing::{parse_rules, create_matcher};
use day19::matching::Matcher;
fn main() {
    let rules = parse_rules(include_str!("rules.txt"));
    let messages = include_str!("messages.txt");
    let matcher = create_matcher(0, &rules);

    let matching = messages.lines()
        .filter(|message| matcher.exactly_matches(message))
        .count();

    println!("{} messages exactly match rule 0.", matching);
}
