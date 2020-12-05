use num_integer::Integer;

fn main() {
    let max_id = include_str!("puzzle_input.txt")
        .lines()
        .map(seat_id)
        .max()
        .unwrap();
    
    println!("The highest seat ID is {}.", max_id);
}

fn partition_lower(range: (usize, usize)) -> (usize, usize) {
    let (min, max) = range;
    (min, min + (max - min).div_floor(&2))
}

fn partition_upper(range: (usize, usize)) -> (usize, usize) {
    let (min, max) = range;
    (min + (max - min).div_ceil(&2), max)
}

fn row(seat: &[char]) -> usize {
    let (min, max) = seat.iter().fold((0, 127), |range, half| match half {
        'F' => partition_lower(range),
        'B' => partition_upper(range),
        _ => panic!("Unexpected character in row specification"),
    });
    assert_eq!(min, max);
    min
}

fn column(seat: &[char]) -> usize {
    let (min, max) = seat.iter().fold((0, 7), |range, half| match half {
        'L' => partition_lower(range),
        'R' => partition_upper(range),
        _ => panic!("Unexpected character in column specification"),
    });
    assert_eq!(min, max);
    min
}

fn seat_id(seat: &str) -> usize {
    let chars: Vec<char> = seat.chars().collect();
    8 * row(&chars[0..=6]) + column(&chars[7..])

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_lower() {
        assert_eq!(partition_lower((0, 127)), (0, 63));
        assert_eq!(partition_lower((32, 63)), (32, 47));
    }

    #[test]
    fn test_partition_upper() {
        assert_eq!(partition_upper((0, 63)), (32, 63));
        assert_eq!(partition_upper((32, 47)), (40, 47));
    }

    #[test]
    fn test_row() {
        let seat: Vec<char> = "FBFBBFF".chars().collect();
        assert_eq!(row(&seat), 44);
    }

    #[test]
    fn test_column() {
        let seat: Vec<char> = "RLR".chars().collect();
        assert_eq!(column(&seat), 5);
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}
