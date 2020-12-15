use std::collections::{HashMap, VecDeque};
fn main() {
    let input = [2,1,10,11,0,6];

    let mut game = MemoryGame::new(&input);
    let part1 = game.nth(2020 - input.len() - 1).unwrap();
    println!("The 2020th number is {}.", part1);

    let mut game = MemoryGame::new(&input);
    let part2 = game.nth(30_000_000 - input.len() - 1).unwrap();
    println!("The 30_000_000th number is {}.", part2);
}

#[derive(Debug)]
struct MemoryGame {
    turn: usize,
    last_number: usize,
    numbers: HashMap<usize, VecDeque<usize>>,
}

impl MemoryGame {
    pub fn new(start: &[usize]) -> MemoryGame {
        let numbers: HashMap<usize, VecDeque<usize>> = start
            .iter()
            .enumerate()
            .map(|(index, &n)| {
                let mut v = VecDeque::with_capacity(2);
                v.push_back(index + 1);
                (n, v)
            })
            .collect();
        let turn = numbers.len();
        let last_number = *start.last().unwrap();

        MemoryGame {
            turn,
            last_number,
            numbers,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.turn += 1;

        let occurrences = &self.numbers[&self.last_number];
        let len = occurrences.len();
        let next_number = if len == 1 {
            0
        } else {
            occurrences[len - 1] - occurrences[len - 2]
        };

        let occurrences = self.numbers.entry(next_number).or_insert_with(||VecDeque::with_capacity(2));

        if occurrences.len() > 1 {
            occurrences.pop_front();
        }
        occurrences.push_back(self.turn);
                
        self.last_number = next_number;

        Some(next_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let mut game = MemoryGame::new(&[0, 3, 6]);
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(1));
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(4));
        assert_eq!(game.next(), Some(0));
    }

    #[test]
    fn part1_example1_2020() {
        let mut game = MemoryGame::new(&[0, 3, 6]);
        assert_eq!(game.nth(2016), Some(436));
    }

    #[test]
    fn part1_example2() {
        let input = [1, 3, 2];
        let mut game = MemoryGame::new(&input);
        assert_eq!(game.nth(2020 - input.len() - 1), Some(1));
    }
}
