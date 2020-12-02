use scan_fmt::scan_fmt;

fn main() {
    let passwords: Vec<(usize, usize, char, String)> = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d} {}: {}", usize, usize, char, String).unwrap())
        .collect();

    let part1_valid = passwords.iter()
        .filter(|(min, max, letter, password)| valid_password_part1(password, *letter, *min, *max))
        .count();

    println!("There are {} valid passwords according to part 1 rules.", part1_valid);

    let part2_valid = passwords.iter()
        .filter(|(pos1, pos2, letter, password)| valid_password_part2(password, *letter, *pos1, *pos2))
        .count();

    println!("There are {} valid passwords according to part 2 rules.", part2_valid);
}

fn valid_password_part1(password: &str, letter: char, min: usize, max: usize) -> bool {
    let count = password.chars()
        .filter(|&c| c == letter)
        .count();
    
    count >= min && count <= max
}

fn valid_password_part2(password: &str, letter: char, pos1: usize, pos2: usize) -> bool {
    let characters: Vec<char> = password.chars().collect();
    let char1 = characters[pos1 - 1];
    let char2 = characters[pos2 - 1];

    (char1 != char2) && (char1 == letter || char2 == letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(valid_password_part1("abcde", 'a', 1, 3), true);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(valid_password_part1("cdefg", 'b', 1, 3), false);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(valid_password_part1("ccccccccc", 'c', 2, 9), true);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(valid_password_part2("abcde", 'a', 1, 3), true);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(valid_password_part2("cdefg", 'b', 1, 3), false);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(valid_password_part2("ccccccccc", 'c', 2, 9), false);
    }
}