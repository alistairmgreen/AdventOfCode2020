use matrix::Matrix;
use std::fmt;

fn main() {
    let mut generation = 0;
    let mut previous_generation = read_grid(include_str!("puzzle_input.txt"));

    loop {
        generation += 1;
        let next_generation = previous_generation.map(|row, column, value| match value {
            Seat::Floor => Seat::Floor,
            Seat::Occupied => {
                if neighbours(row, column, &previous_generation) >= 4 {
                    Seat::Vacant
                } else {
                    Seat::Occupied
                }
            }
            Seat::Vacant => {
                if neighbours(row, column, &previous_generation) == 0 {
                    Seat::Occupied
                } else {
                    Seat::Vacant
                }
            }
        });

        if next_generation == previous_generation {
            println!("Arrangement stabilised at generation {}", generation);
            break;
        }

        previous_generation = next_generation;
    }

    let occupied_count = previous_generation
        .iter()
        .filter(|&&seat| seat == Seat::Occupied)
        .count();
    println!("{} seats are occupied:", occupied_count);
    println!("{}", previous_generation);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Occupied,
    Vacant,
    Floor,
}

impl From<char> for Seat {
    fn from(character: char) -> Self {
        match character {
            '#' => Seat::Occupied,
            'L' => Seat::Vacant,
            _ => Seat::Floor,
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Seat::Occupied => write!(f, "#"),
            Seat::Vacant => write!(f, "L"),
            Seat::Floor => write!(f, "."),
        }
    }
}

fn read_grid(grid: &str) -> Matrix<Seat> {
    let seats: Vec<_> = grid.lines().collect();
    let width = seats[0].len();

    let seats: Vec<Seat> = seats
        .into_iter()
        .flat_map(|line| line.trim().chars())
        .map(|c| c.into())
        .collect();
    Matrix::from_vec(seats, width)
}

fn neighbours(row: usize, column: usize, grid: &Matrix<Seat>) -> usize {
    let min_row = if row > 0 { row - 1 } else { 0 };
    let max_row = if row < grid.height() - 1 {
        row + 1
    } else {
        row
    };

    let min_column = if column > 0 { column - 1 } else { 0 };
    let max_column = if column < grid.width() - 1 {
        column + 1
    } else {
        column
    };

    let mut occupied = 0;
    for y in min_row..=max_row {
        for x in min_column..=max_column {
            if y == row && x == column {
                continue;
            }

            if grid[y][x] == Seat::Occupied {
                occupied += 1;
            }
        }
    }

    occupied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours1() {
        let grid = read_grid(
"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        );
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);

        assert_eq!(neighbours(0, 0, &grid), 2);
        assert_eq!(neighbours(9, 9, &grid), 2);
    }
}
