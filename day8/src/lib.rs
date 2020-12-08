use std::error::Error;
use std::fmt;
use std::str::FromStr;
use scan_fmt::scan_fmt;

#[derive(Debug)]
pub struct InvalidInstructionError {
    pub instruction: String,
}

impl fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid instruction: {}", self.instruction)
    }
}

impl Error for InvalidInstructionError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Instruction {
    Nop,
    Accumulator(isize),
    Jump(isize),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, argument) = scan_fmt!(s, "{} {d}", String, isize)?;
        match operation.as_str() {
            "nop" => Ok(Instruction::Nop),
            "acc" => Ok(Instruction::Accumulator(argument)),
            "jmp" => Ok(Instruction::Jump(argument)),
            unknown => Err(Box::new(InvalidInstructionError { instruction: unknown.to_string() }))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nop() {
        let instruction: Instruction = "nop +0".parse().unwrap();
        assert_eq!(instruction, Instruction::Nop);
    }

    #[test]
    fn parse_acc() {
        let instruction: Instruction = "acc +1".parse().unwrap();
        assert_eq!(instruction, Instruction::Accumulator(1));
    }

    #[test]
    fn parse_jmp() {
        let instruction: Instruction = "jmp -3".parse().unwrap();
        assert_eq!(instruction, Instruction::Jump(-3));
    }

    #[test]
    fn parse_invalid() {
        let instruction: Result<Instruction, Box<dyn Error>> = "invalid".parse();
        assert!(instruction.is_err());
    }
}