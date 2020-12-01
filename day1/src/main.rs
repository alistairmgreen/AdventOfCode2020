fn main() {
    let mut numbers: Vec<i32> = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    numbers.sort_unstable();
    let numbers = numbers;

    match pair_adding_to(&numbers, 2020) {
        Some((x, y)) => {
            println!("{} + {} = 2020", x, y);
            println!("{} * {} = {}", x, y, x * y);
        }
        None => {
            println!("No solution found.");
        }
    }
}

fn pair_adding_to(numbers: &[i32], target: i32) -> Option<(i32, i32)> {
    let max_index = numbers.len() - 1;
    for (index, &n) in numbers.iter().enumerate() {
        let required = target - n;
        if index < max_index && numbers[index+1..].binary_search(&required).is_ok() {
            return Some((n, required));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_adding_to_example_1() {
        let numbers = vec![299, 366, 675, 979, 1456, 1721];
        assert_eq!(pair_adding_to(&numbers, 2020), Some((299, 1721)));
    }
}