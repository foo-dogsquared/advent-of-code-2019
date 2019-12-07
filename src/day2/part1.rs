// COMMENTS FOR THIS CHALLENGE: 
// 
// This one is pretty easy. 
// I was able to make up a solution right away by thinking of the program as a parser. 
// 
// This one has imperfections but it is quite nice to get it on first try, if you ask me. 

use std::fs;
use crate::{ Result, Error };

pub enum Opcode {
    Add(i64, i64, i64), 
    Multiply(i64, i64, i64), 
    Halt, 
}

pub struct Intcode {
    input: Vec<i64>, 
    index: usize, 
}

impl Intcode {
    /// Create a new Intcode program instance.
    pub fn new (input: Vec<i64>) -> Self {
        Self {
            input, 
            index: 0, 
        }
    }

    /// Get the opcode with the current index. 
    /// If the opcode is not valid (i.e., is not 0, 1, or 99), it will return an error. 
    pub fn get_opcode (&self) -> Option<Opcode> {
        let intcode_bit = self.input.get(self.index).unwrap().clone();

        match intcode_bit {
            1 => {
                let operand_position_1 = self.input.get(self.index + 1).unwrap().clone();
                let operand_position_2 = self.input.get(self.index + 2).unwrap().clone();
                let output_position = self.input.get(self.index + 3).unwrap().clone();

                Some(Opcode::Add(operand_position_1, operand_position_2, output_position))
            }, 
            2 => {
                let operand_position_1 = self.input.get(self.index + 1).unwrap().clone();
                let operand_position_2 = self.input.get(self.index + 2).unwrap().clone();
                let output_position = self.input.get(self.index + 3).unwrap().clone();

                Some(Opcode::Multiply(operand_position_1, operand_position_2, output_position))
            }, 
            99 => Some(Opcode::Halt), 
            _ => None, 
        }
    }

    pub fn next (&mut self) -> Result<()> {
        self.index += 4;

        match self.input.get(self.index) {
            Some(v) => Ok(()), 
            None => Err(Box::new(Error::ValueError)), 
        }
    }

    pub fn parse (&mut self) -> Result<()> {
        while let Some(op) = self.get_opcode() {
            match op {
                Opcode::Add (a, b, c) => self.input[c as usize] = self.input[a as usize] + self.input[b as usize], 
                Opcode::Multiply (a, b, c) => self.input[c as usize] = self.input[a as usize] * self.input[b as usize], 
                Opcode::Halt => break, 
            }

            self.next()?;
        }

        Ok(())
    }
}

pub fn main () -> Result<Vec<i64>> {
    let content = fs::read_to_string("./inputs/day2.txt")?;
    let mut input: Vec<i64> = content.split(',').map(| v | v.parse().unwrap() ).collect();

    // this is described in the instructions as 'before running the computer' set
    input[1] = 12;
    input[2] = 2;

    let mut intcode = Intcode::new(input);
    intcode.parse()?;
    
    Ok(intcode.input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    pub fn basic_example() -> Result<()> {
        let mut intcode = Intcode::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        intcode.parse()?;

        assert_eq!(intcode.input, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        Ok(())
    }
}