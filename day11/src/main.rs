use matrix::Matrix;
use std::fmt;

fn main() {
    let grid = read_grid(include_str!("puzzle_input.txt"));
    let (generations, occupied) = part1(&grid);

    println!(
        "Part 1: Pattern stabilises at generation {}. {} seats are occupied.",
        generations, occupied
    );

    let (generations, occupied) = part2(&grid);

    println!(
        "Part 2: Pattern stabilises at generation {}. {} seats are occupied.",
        generations, occupied
    );
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

fn part1(grid: &Matrix<Seat>) -> (usize, usize) {
    simulate(grid, |row, column, value, previous| match value {
        Seat::Floor => Seat::Floor,
        Seat::Occupied => {
            if neighbours(row, column, &previous) >= 4 {
                Seat::Vacant
            } else {
                Seat::Occupied
            }
        }
        Seat::Vacant => {
            if neighbours(row, column, &previous) == 0 {
                Seat::Occupied
            } else {
                Seat::Vacant
            }
        }
    })
}

fn part2(grid: &Matrix<Seat>) -> (usize, usize) {
    simulate(grid, |row, column, value, previous| match value {
        Seat::Floor => Seat::Floor,
        Seat::Occupied => {
            if in_sight(row, column, &previous) >= 5 {
                Seat::Vacant
            } else {
                Seat::Occupied
            }
        }
        Seat::Vacant => {
            if in_sight(row, column, &previous) == 0 {
                Seat::Occupied
            } else {
                Seat::Vacant
            }
        }
    })
}

fn simulate(
    grid: &Matrix<Seat>,
    rule: impl Fn(usize, usize, &Seat, &Matrix<Seat>) -> Seat,
) -> (usize, usize) {
    let mut previous_generation = grid.clone();
    let mut generation = 0;
    loop {
        generation += 1;
        let next_generation = previous_generation
            .map(|row, column, value| rule(row, column, value, &previous_generation));

        if next_generation == previous_generation {
            break;
        }

        previous_generation = next_generation;
    }

    let occupied_count = previous_generation
        .iter()
        .filter(|&&seat| seat == Seat::Occupied)
        .count();

    (generation, occupied_count)
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

fn in_sight(row: usize, column: usize, grid: &Matrix<Seat>) -> usize {
    let mut visible = 0;

    if look(&grid, row, column, | y, x | (y, x-1)) == Seat::Occupied {
        visible += 1;
    }

    if look(&grid, row, column, | y, x | (y, x+1)) == Seat::Occupied {
        visible += 1;
    }

    if look(&grid, row, column, | y, x | (y-1, x)) == Seat::Occupied {
        visible += 1;
    }

    if look(&grid, row, column, | y, x | (y+1, x)) == Seat::Occupied {
        visible += 1;
    }

    if look(&grid, row, column, | y, x | (y-1, x-1)) == Seat::Occupied {
        visible += 1;
    }

    if look(&grid, row, column, | y, x | (y-1, x+1)) == Seat::Occupied {
        visible += 1;
    }

    if look(&grid, row, column, | y, x | (y+1, x-1)) == Seat::Occupied {
        visible += 1;
    }

    if look(&grid, row, column, | y, x | (y+1, x+1)) == Seat::Occupied {
        visible += 1;
    }

    visible
}

fn look(grid: &Matrix<Seat>, row: usize, column: usize, step: impl Fn(isize, isize) -> (isize, isize)) -> Seat {
    let (mut y, mut x) = step(row as isize, column as isize);

    while x >= 0 && (x as usize) < grid.width() && y >= 0 && (y as usize) < grid.height() {
        match grid[y as usize][x as usize] {
            Seat::Floor => {
                let next = step(y, x);
                y = next.0;
                x = next.1;
            }
            Seat::Occupied => {
                return Seat::Occupied;
            }
            Seat::Vacant => {
                return Seat::Vacant;
            }
        }
    }

    Seat::Floor
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

    #[test]
    fn test_part1() {
        let grid = read_grid("L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL");

        let (_, occupied) = part1(&grid);
        assert_eq!(occupied, 37);
    }

    #[test]
    fn test_part2() {
        let grid = read_grid("L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL");

        let (_, occupied) = part2(&grid);
        assert_eq!(occupied, 26);
    }
}
