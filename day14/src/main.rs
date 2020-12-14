use scan_fmt::scan_fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

fn main() -> Result<(), InvalidBitmaskError> {
    let program: Vec<&str> = include_str!("puzzle_input.txt").lines().collect();

    let result = run(&program)?;

    println!("Part 1 result: {}", result);

    let result = run_part2(&program)?;
    println!("Part 2 result: {}", result);

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InvalidBitmaskError {
    InvalidCharacter(char),
    InvalidLength(usize),
}

impl fmt::Display for InvalidBitmaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidBitmaskError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            InvalidBitmaskError::InvalidLength(l) => write!(f, "Invalid length: {}", l),
        }
    }
}

impl Error for InvalidBitmaskError {}

struct Bitmask {
    set_bits: u64,
    clear_bits: u64,
}

impl Bitmask {
    pub fn apply(&self, value: u64) -> u64 {
        (value | self.set_bits) & self.clear_bits
    }
}

impl Default for Bitmask {
    fn default() -> Self {
        Bitmask {
            set_bits: 0,
            clear_bits: u64::MAX,
        }
    }
}

impl FromStr for Bitmask {
    type Err = InvalidBitmaskError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.len();
        if length != 36 {
            return Err(InvalidBitmaskError::InvalidLength(length));
        }

        let mut mask: Bitmask = Default::default();
        for (index, bit) in s.chars().rev().enumerate() {
            match bit {
                '0' => {
                    mask.clear_bits ^= 1 << index;
                }
                '1' => {
                    mask.set_bits |= 1 << index;
                }
                'X' => {}
                other => return Err(InvalidBitmaskError::InvalidCharacter(other)),
            }
        }

        Ok(mask)
    }
}

struct Bitmask2 {
    set_bits: u64,
    floating_bits: Vec<usize>,
}

impl Bitmask2 {
    pub fn apply(&self, value: u64) -> Vec<u64> {
        let seed = value | self.set_bits;
        let mut outputs = Vec::with_capacity(1 << self.floating_bits.len());
        Bitmask2::apply_floating(seed, &self.floating_bits, &mut outputs);

        outputs
    }

    fn apply_floating(seed: u64, indices: &[usize], outputs: &mut Vec<u64>) {
        if indices.is_empty() {
            outputs.push(seed);
        } else {
            let mask: u64 = 1 << indices[0];
            let bit_set = seed | mask;
            let bit_cleared = bit_set ^ mask;
            if indices.len() > 1 {
                Bitmask2::apply_floating(bit_set, &indices[1..], outputs);
                Bitmask2::apply_floating(bit_cleared, &indices[1..], outputs);
            } else {
                outputs.push(bit_set);
                outputs.push(bit_cleared);
            }

        }
    }
}

impl Default for Bitmask2 {
    fn default() -> Self {
        Bitmask2 {
            set_bits: 0,
            floating_bits: Vec::new(),
        }
    }
}

impl FromStr for Bitmask2 {
    type Err = InvalidBitmaskError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.len();
        if length != 36 {
            return Err(InvalidBitmaskError::InvalidLength(length));
        }

        let mut mask: Bitmask2 = Default::default();
        for (index, bit) in s.chars().rev().enumerate() {
            match bit {
                '0' => {}
                '1' => {
                    mask.set_bits |= 1 << index;
                }
                'X' => {
                    mask.floating_bits.push(index);
                }
                other => return Err(InvalidBitmaskError::InvalidCharacter(other)),
            }
        }

        Ok(mask)
    }
}


fn run(program: &[&str]) -> Result<u64, InvalidBitmaskError> {
    let mut memory = BTreeMap::new();
    let mut mask: Bitmask = Default::default();

    for instruction in program {
        if let Ok(m) = scan_fmt!(instruction, "mask = {}", String) {
            mask = m.parse()?;
        } else if let Ok((address, value)) = scan_fmt!(instruction, "mem[{d}] = {d}", u64, u64) {
            memory.insert(address, mask.apply(value));
        }
    }

    let total = memory.values().sum();
    Ok(total)
}

fn run_part2(program: &[&str]) -> Result<u64, InvalidBitmaskError> {
    let mut memory = BTreeMap::new();
    let mut mask: Bitmask2 = Default::default();

    for instruction in program {
        if let Ok(m) = scan_fmt!(instruction, "mask = {}", String) {
            mask = m.parse()?;
        } else if let Ok((address, value)) = scan_fmt!(instruction, "mem[{d}] = {d}", u64, u64) {
            let addresses = mask.apply(address);
            for address in addresses {
                memory.insert(address, value);
            }
        }
    }

    let total = memory.values().sum();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmask() {
        let mask: Bitmask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
        assert_eq!(mask.apply(11), 73);
    }

    #[test]
    fn test_part1() {
        let program: Vec<&str> = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0"
            .lines()
            .collect();
        let result = run(&program);

        assert_eq!(result, Ok(165));
    }

    #[test]
    fn test_bitmask2() {
        let mask: Bitmask2 = "000000000000000000000000000000X1001X".parse().unwrap();
        let mut values = mask.apply(42);
        values.sort_unstable();
        assert_eq!(values, vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_part2() {
        let program: Vec<&str> = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1".lines().collect();
        let result = run_part2(&program);
        assert_eq!(result, Ok(208));
    }
}
