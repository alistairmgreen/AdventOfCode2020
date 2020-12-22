use std::collections::VecDeque;
fn main() {
    let mut player1 = VecDeque::from(vec![
        26, 16, 33, 8, 5, 46, 12, 47, 39, 27, 50, 10, 34, 20, 23, 11, 43, 14, 18, 1, 48, 28, 31,
        38, 41,
    ]);

    let mut player2 = VecDeque::from(vec![
        45, 7, 9, 4, 15, 19, 49, 3, 36, 25, 24, 2, 21, 37, 35, 44, 29, 13, 32, 22, 17, 30, 42, 40,
        6,
    ]);

    play(&mut player1, &mut player2);

    if player1.is_empty() {
        println!("Player 2 wins. Score: {}", score(&player2));
    } else {
        println!("Player 1 wins. Score: {}", score(&player1));
    }
}

fn play(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) {
    while !player1.is_empty() && !player2.is_empty() {
        let top1 = player1.pop_front().unwrap();
        let top2 = player2.pop_front().unwrap();
        if top1 > top2 {
            player1.push_back(top1);
            player1.push_back(top2);
        } else {
            player2.push_back(top2);
            player2.push_back(top1);
        }
    }
}

fn score(cards: &VecDeque<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(index, value)| (index + 1) * value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let mut player1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let mut player2 = VecDeque::from(vec![5, 8, 4, 7, 10]);

        play(&mut player1, &mut player2);

        assert!(player1.is_empty());
        assert_eq!(player2, VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]));
    }

    #[test]
    fn test_score() {
        let cards = VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);

        assert_eq!(score(&cards), 306);
    }
}
