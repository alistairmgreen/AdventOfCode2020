use scan_fmt::scan_fmt;

fn main() {
    let passwords = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d} {}: {}", usize, usize, char, String).unwrap())
        .filter(|(min, max, letter, password)| valid_password(password, *letter, *min, *max))
        .count();

    println!("There are {} valid passwords.", passwords);
}

fn valid_password(password: &str, letter: char, min: usize, max: usize) -> bool {
    let count = password.chars()
        .filter(|&c| c == letter)
        .count();
    
    count >= min && count <= max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(valid_password("abcde", 'a', 1, 3), true);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(valid_password("cdefg", 'b', 1, 3), false);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(valid_password("ccccccccc", 'c', 2, 9), true);
    }
}