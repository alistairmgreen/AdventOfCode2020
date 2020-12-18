fn main() {
    let homework = include_str!("puzzle_input.txt");

    let total: i64 = homework.lines().map(eval).sum();

    println!("The sum of all expressions is {}.", total);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    pub fn apply(&self, a: i64, b: i64) -> i64 {
        match *self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

fn eval(expression: &str) -> i64 {
    let mut accumulator = 0;
    let mut operator = Operator::Add;
    let mut characters = expression.chars();

    while let Some(c) = characters.next() {
        match c {
            '+' => {
                operator = Operator::Add;
            }
            '*' => {
                operator = Operator::Multiply;
            }
            n if n.is_numeric() => {
                let value: i64 = n.to_string().parse().unwrap();
                accumulator = operator.apply(accumulator, value);
            }
            '(' => {
                let buffer = read_to_bracket(&mut characters);
                let value = eval(&buffer);
                accumulator = operator.apply(accumulator, value);
            }
            _ => {}
        }
    }

    accumulator
}

fn read_to_bracket(characters: &mut impl Iterator<Item = char>) -> String {
    let mut buffer = String::new();
    let mut depth = 1;
    for x in characters {
        match x {
            '(' => {
                depth += 1;
                buffer.push(x);
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    break;
                } else {
                    buffer.push(x);
                }
            }
            _ => {
                buffer.push(x);
            }
        }
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_no_brackets() {
        assert_eq!(5, eval("2 + 3"));
        assert_eq!(25, eval("2 + 3 * 5"));
        assert_eq!(71, eval("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn eval_brackets() {
        assert_eq!(51, eval("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(
            13632,
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
