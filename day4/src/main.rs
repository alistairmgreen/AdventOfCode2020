use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let passports: Vec<_> = include_str!("puzzle_input.txt")
        .lines()
        .batching(read_passport)
        .filter(has_required_fields)
        .collect();
    println!("{} passports have all required fields.", passports.len());

    let valid = passports
        .iter()
        .filter(|passport| is_valid(passport))
        .count();

    println!("{} passports are valid.", valid);
}

fn read_passport<'a, T>(batch: &mut T) -> Option<HashMap<&'a str, &'a str>>
where
    T: Iterator<Item = &'a str> + Clone,
{
    let passport: HashMap<&'a str, &'a str> = batch
        .take_while_ref(|line| !line.is_empty())
        .flat_map(|line| line.split_whitespace())
        .map(|item| {
            let parts: Vec<_> = item.split(':').collect();
            (parts[0], parts[1])
        })
        .collect();

    batch.next();

    if passport.is_empty() {
        None
    } else {
        Some(passport)
    }
}

fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required.iter().all(|key| passport.contains_key(key))
}

fn is_valid(passport: &HashMap<&str, &str>) -> bool {
    valid_birth_year(passport["byr"])
        && valid_issue_year(passport["iyr"])
        && valid_expiration_year(passport["eyr"])
        && valid_height(passport["hgt"])
        && valid_hair_colour(passport["hcl"])
        && valid_eye_colour(passport["ecl"])
        && valid_pid(passport["pid"])
}

fn is_four_digits(value: &str, min: u16, max: u16) -> bool {
    lazy_static! {
        static ref FOUR_DIGITS: Regex = Regex::new(r"^\d{4}$").unwrap();
    }

    if FOUR_DIGITS.is_match(value) {
        let number: u16 = value.parse().unwrap();
        number >= min && number <= max
    } else {
        false
    }
}

fn valid_birth_year(year: &str) -> bool {
    is_four_digits(year, 1920, 2002)
}

fn valid_expiration_year(year: &str) -> bool {
    is_four_digits(year, 2020, 2030)
}

fn valid_issue_year(year: &str) -> bool {
    is_four_digits(year, 2010, 2020)
}

fn valid_height(height: &str) -> bool {
    lazy_static! {
        static ref HEIGHT: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }

    match HEIGHT.captures(height) {
        Some(captures) => {
            let value: u16 = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .expect("Cannot parse numeric height value");
            let unit = captures.get(2).unwrap().as_str();
            match unit {
                "cm" => value >= 150 && value <= 193,
                "in" => value >= 59 && value <= 76,
                _ => unreachable!("Would not match regex"),
            }
        }
        None => false,
    }
}

fn valid_hair_colour(colour: &str) -> bool {
    lazy_static! {
        static ref HEX_COLOUR: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    }
    HEX_COLOUR.is_match(colour)
}

fn valid_eye_colour(colour: &str) -> bool {
    matches!(
        colour,
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    )
}

fn valid_pid(pid: &str) -> bool {
    lazy_static! {
        static ref NINE_DIGITS: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    NINE_DIGITS.is_match(pid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eye_colour_valid() {
        assert_eq!(valid_eye_colour("brn"), true);
    }

    #[test]
    fn eye_colour_invalid() {
        assert_eq!(valid_eye_colour("wat"), false);
    }

    #[test]
    fn pid_valid() {
        assert_eq!(valid_pid("000000001"), true);
    }

    #[test]
    fn pid_invalid() {
        assert_eq!(valid_pid("0123456789"), false);
    }

    #[test]
    fn test_height() {
        assert_eq!(valid_height("60in"), true);
        assert_eq!(valid_height("190cm"), true);
        assert_eq!(valid_height("190in"), false);
        assert_eq!(valid_height("190"), false);
        assert_eq!(valid_height("nan"), false);
    }

    #[test]
    fn test_hair_colour() {
        assert_eq!(valid_hair_colour("#123abc"), true);
        assert_eq!(valid_hair_colour("#123abz"), false);
        assert_eq!(valid_hair_colour("123abc"), false);
    }
}
