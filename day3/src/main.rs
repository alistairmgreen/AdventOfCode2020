fn main() {
    let map: Vec<&str> = include_str!("puzzle_input.txt").lines().collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product = 1;

    for (right, down) in slopes {
        let n = trees(&map, right, down);
        println!("Right {}, down {}: {} trees", right, down, n);
        product *= n;
    }

    println!("\nProduct = {}", product);
}

fn trees(map: &[&str], right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut x = 0;

    for row in map.iter().step_by(down) {
        let chars: Vec<char> = row.chars().collect();
        if chars[x % chars.len()] == '#' {
            trees += 1;
        }

        x += right;
    }

    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_example() -> Vec<&'static str> {
        include_str!("part1_example.txt").lines().collect()
    }

    #[test]
    fn part1_example() {
        let map = load_example();
        assert_eq!(trees(&map, 3, 1), 7);
    }

    #[test]
    fn part2_example1() {
        let map = load_example();
        assert_eq!(trees(&map, 1, 1), 2);
    }

    #[test]
    fn part2_example3() {
        let map = load_example();
        assert_eq!(trees(&map, 5, 1), 3);
    }

    #[test]
    fn part2_example4() {
        let map = load_example();
        assert_eq!(trees(&map, 7, 1), 4);
    }

    #[test]
    fn part2_example5() {
        let map = load_example();
        assert_eq!(trees(&map, 1, 2), 2);
    }
}
