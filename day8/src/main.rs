use day8::Instruction;
use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let program = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()?;
    
    let accumulator = run(&program);

    println!("Accumulator = {}", accumulator);

    Ok(())
}

fn run(instructions: &[Instruction]) -> isize {
    let mut accumulator = 0;
    let mut pointer = 0;
    let mut visited = HashSet::new();

    while let Some(instruction) = instructions.get(pointer) {
        match instruction {
            Instruction::Accumulator(a) => {
                accumulator += a;
                pointer += 1;
            }
            Instruction::Jump(j) => {
                pointer = (pointer as isize + j) as usize;
            }
            Instruction::Nop => {
                pointer += 1;
            }
        }

        if !visited.insert(pointer) {
            break;
        }
    }
    
    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let instructions: Vec<Instruction> = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6".lines()
            .map(|line| line.trim().parse())
            .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()
            .unwrap();

        let acc = run(&instructions);
        assert_eq!(acc, 5);
    }
}