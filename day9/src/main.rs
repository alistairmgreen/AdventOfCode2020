fn main() {
    let numbers: Vec<i64> = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    match find_invalid(&numbers, 25) {
        Some(n) => {
            println!("{} is invalid.", n);

            if let Some(range) = contiguous_summing_to(&numbers, n) {
                println!("Set of {} contiguous numbers adds up to {}.", range.len(), n);
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                println!("{} + {} = {}", min, max, min + max);
            }
        }
        None => {
            println!("No solution found for part 1.");
        }
    }
}

fn pair_adding_to(numbers: &[i64], target: i64) -> Option<(i64, i64)> {
    for (index, &n) in numbers[..numbers.len() - 1].iter().enumerate() {
        let required = target - n;
        if numbers[index + 1..].binary_search(&required).is_ok() {
            return Some((n, required));
        }
    }

    None
}

fn find_invalid(numbers: &[i64], window: usize) -> Option<i64> {
    for index in window..numbers.len() {
        let mut preamble = numbers[index - window..index].to_owned();
        preamble.sort_unstable();
        if pair_adding_to(&preamble, numbers[index]).is_none() {
            return Some(numbers[index]);
        }
    }

    None
}

fn contiguous_summing_to(numbers: &[i64], target: i64) -> Option<&[i64]> {
    for length in 2..=numbers.len() {
        if let Some(portion) = numbers
            .windows(length)
            .find(|&window| window.iter().sum::<i64>() == target)
        {
            return Some(portion);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_invalid() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_invalid(&numbers, 5), Some(127));
    }

    #[test]
    fn test_contiguous_summing_to() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let result = contiguous_summing_to(&numbers, 127).unwrap();
        assert_eq!(result, &[15, 25, 47, 40]);
    }
}
