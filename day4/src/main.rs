use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let passports: Vec<_> = include_str!("example.txt")
        .lines()
        .batching(|batch| read_passport(batch))
        .collect();
    for passport in &passports {
        println!("{:?}", passport);
        if is_valid(&passport) {
            println!("VALID");
        } else {
            println!("INVALID");
        }
        println!("\n");
    }
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

fn is_valid(passport: &HashMap<&str, &str>) -> bool {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required.iter().all(|key| passport.contains_key(key))
}
