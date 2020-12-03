fn main() {
    let map: Vec<&str> = include_str!("puzzle_input.txt").lines().collect();

    let part1 = trees(&map, 3, 1);

    println!("Part 1: We encounter {} trees.", part1);
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

    #[test]
    fn part1_example() {
        let map: Vec<&str> = include_str!("part1_example.txt").lines().collect();
        assert_eq!(trees(&map, 3, 1), 7);
    }
}
