use std::error::Error;
use std::fmt;
use std::num;
use std::str::FromStr;

fn main() -> Result<(), InvalidInstructionError> {
    let voyage = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Instruction>, InvalidInstructionError>>()?;
    
    let (x, y) = sail(&voyage)?;
    let manhattan = x.abs() + y.abs();

    println!("Manhattan distance = {}", manhattan);
    
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug, Eq, PartialEq)]
struct InvalidInstructionError {
    message: String,
}

impl Error for InvalidInstructionError {}

impl fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<num::ParseIntError> for InvalidInstructionError {
    fn from(e: num::ParseIntError) -> Self {
        InvalidInstructionError {
            message: format!("{}", e),
        }
    }
}

impl FromStr for Instruction {
    type Err = InvalidInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_owned();
        if s.len() < 2 {
            return Err(InvalidInstructionError {
                message: format!("Instruction too short: {}", s),
            });
        }

        let value: i32 = s.split_off(1).parse()?;
        let letter = s.chars().next().unwrap();
        match letter {
            'N' => Ok(Instruction::North(value)),
            'S' => Ok(Instruction::South(value)),
            'E' => Ok(Instruction::East(value)),
            'W' => Ok(Instruction::West(value)),
            'L' => Ok(Instruction::Left(value)),
            'R' => Ok(Instruction::Right(value)),
            'F' => Ok(Instruction::Forward(value)),
            unknown => Err(InvalidInstructionError {
                message: format!("Unexpected instruction '{}'.", unknown),
            }),
        }
    }
}

fn sail(instructions: &[Instruction]) -> Result<(i32, i32), InvalidInstructionError> {
    let mut x = 0;
    let mut y = 0;
    let mut heading = 90;

    for instruction in instructions {
        match instruction {
            Instruction::North(n) => {
                y += n;
            }
            Instruction::South(n) => {
                y -= n;
            }
            Instruction::East(n) => {
                x += n;
            }
            Instruction::West(n) => {
                x -= n;
            }
            Instruction::Left(n) => {
                heading = (heading - n).rem_euclid(360)
            }
            Instruction::Right(n) => {
                heading = (heading + n).rem_euclid(360)
            }
            Instruction::Forward(n) => match heading {
                0 => y += n,
                90 => x += n,
                180 => y -= n,
                270 => x -= n,
                other => {
                    return Err(InvalidInstructionError {
                        message: format!("Unexpected heading {}", other),
                    })
                }
            },
        }
    }

    Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let i = "F10".parse::<Instruction>();
        assert_eq!(i, Ok(Instruction::Forward(10)));
    }

    #[test]
    fn test_turn_left() {
        let heading: i32 = (0i32 - 90i32).rem_euclid(360);
        assert_eq!(heading, 270);
    }

    #[test]
    fn test_part1() {
        let instructions = vec![
            Instruction::Forward(10),
            Instruction::North(3),
            Instruction::Forward(7),
            Instruction::Right(90),
            Instruction::Forward(11),
        ];

        assert_eq!(sail(&instructions), Ok((17, -8)))
    }
}
