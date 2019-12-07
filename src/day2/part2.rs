// COMMENTS FOR THIS CHALLENGE: 
// 
// I initially thought of this challenging until I read and understood the instructions more carefully 
// out of desparation. 
// It only needs the first two inputs while I'm here figuring out how to parse and verify an instruction block 
// for an intcode program. 
// I even thought of implementing an undo/redo system with states. 
// I'll leave the code as it is, providing an example of shame for not reading too well. 
// 
// This would be solved in minutes if I read it right. 
// I hate myself sometimes. >:-\ 
// 
// At least I made a pretty clean codebase, if I say so myself. 
// It is separated by its functions that does one thing well, it has atomic operations, and all. 
// It even comes with some test cases... oooooh. 
// This challenge cause me to love parsers a bit (a little bit). 

use std::fs;
use crate::{ Result, Error };

#[derive(Debug)]
pub enum Opcode {
    Add(Vec<i64>), 
    Multiply(Vec<i64>), 
    Halt, 
}

impl Opcode {
    /// Simply checks if the Opcode is the halting instruction. 
    pub fn is_halt (&self) -> bool {
        match &self {
            Opcode::Halt => true, 
            _ => false
        }
    }
}

pub struct Intcode {
    program: Vec<i64>, 
    pointer: usize, 
}

impl Intcode {
    /// Create a new Intcode program instance.
    pub fn new (program: Vec<i64>) -> Self {
        Self {
            program, 
            pointer: 0, 
        }
    }

    /// Move on to the next instruction. 
    /// 
    /// It will result in an error once it goes out of bounds. 
    pub fn next (&mut self) -> Result<()> {
        let current_instruction_vector = self.get_instruction_vector()?;
        
        self.pointer += current_instruction_vector.len();

        match self.program.get(self.pointer) {
            Some(_v) => Ok(()), 
            None => Err(Box::new(Error::ValueError)), 
        }
    }

    /// Parses the intcode. 
    /// 
    /// Take note this function mutates the intcode program. 
    pub fn parse (&mut self) -> Result<()> {
        while let Some(op) = self.get_opcode() {
            if op.is_halt() {
                break;
            }

            self.consume_opcode(op)?;
            
            self.next()?;
        }

        Ok(())
    }

    /// Get the instruction of the current pointer as a raw vector. 
    /// 
    /// The instruction vector consists of the opcode and its parameter. 
    /// This function does not check if the instruction is invalid. 
    pub fn get_instruction_vector (&self) -> Result<Vec<i64>> {
        let intcode_bit = self.program.get(self.pointer).unwrap().clone();
        let instruction_vector = match intcode_bit {
            1 => self.program.get(self.pointer..self.pointer + 4), 
            2 => self.program.get(self.pointer..self.pointer + 4), 
            _ => self.program.get(self.pointer..self.pointer + 1), 
        };

        match instruction_vector { 
            Some(slice) => Ok(slice.to_vec()), 
            None => Err(Box::new(Error::ValueError))
        }
    }

    /// Get the opcode with the current index. 
    /// If the opcode is not valid (i.e., is not 0, 1, or 99) or has an instruction vector that is out of bounds, it will return nothing. 
    pub fn get_opcode (&self) -> Option<Opcode> {
        let current_instruction_vector = match self.get_instruction_vector() {
            Ok(v) => v, 
            Err(_e) => return None,  
        };
        let opcode = current_instruction_vector[0];

        match opcode {
            1 => Some(Opcode::Add(current_instruction_vector)), 
            2 => Some(Opcode::Multiply(current_instruction_vector)), 
            99 => Some(Opcode::Halt), 
            _ => None, 
        }
    }

    /// Checks if the current pointer is at a valid instruction block. 
    pub fn is_current_pointer_valid (&self) -> bool {
        self.get_opcode().is_some()
    }

    /// Consume the opcode and execute the instructions. 
    /// For halting the program, you should make up the code for it since it does nothing. 
    /// 
    /// This code will panic if the execution occurs out of bounds of the intcode program. 
    /// (TODO: Make the code return an error instead of panicking.)
    pub fn consume_opcode (&mut self, op: Opcode) -> Result<()> {
        match op {
            Opcode::Add (vec) => self.program[vec[3] as usize] = self.program[vec[1] as usize] + self.program[vec[2] as usize], 
            Opcode::Multiply (vec) => self.program[vec[3] as usize] = self.program[vec[1] as usize] * self.program[vec[2] as usize], 
            Opcode::Halt => (),  
        }

        Ok(())
    }
}

pub fn main () -> Result<Vec<i64>> {
    let content = fs::read_to_string("./inputs/day2.txt")?;
    let mut input: Vec<i64> = content.split(',').map(| v | v.parse().unwrap() ).collect();

    // this is described in the instructions as 'before running the computer' set
    const RESULT: i64 = 19690720;   
    loop {
        let mut intcode = Intcode::new(input.clone());
        intcode.parse()?;

        if intcode.program[0] == RESULT {
            return Ok(intcode.program);
        }

        if input[1] >= input.len() as i64 && input[2] >= input.len() as i64 {
            return Err(Box::new(Error::ValueError));
        }

        if input[1] <= 99 {
            input[1] += 1;
        }
        
        // setting the second value for permutations
        // think of it like how a clock goes back to 0 each time the second hand goes to 60
        if input[1] >= 99 && input[2] < 99 {
            input[2] += 1;
            input[1] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    pub fn basic_example() -> Result<()> {
        let mut intcode = Intcode::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        assert_eq!(intcode.get_instruction_vector()?, [1, 9, 10, 3]);

        intcode.parse()?;

        assert_eq!(intcode.program, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        Ok(())
    }

    #[test]
    pub fn basic_internal_inspection() -> Result<()> {
        let mut intcode = Intcode::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        // parsing the intcode
        assert_eq!(intcode.get_instruction_vector()?, [1, 9, 10, 3]);
        assert_eq!(intcode.is_current_pointer_valid(), true);
        intcode.next()?;
        
        assert_eq!(intcode.get_instruction_vector()?, [2, 3, 11, 0]);
        assert_eq!(intcode.is_current_pointer_valid(), true);
        intcode.next()?;

        assert_eq!(intcode.get_instruction_vector()?, [99]);
        assert_eq!(intcode.is_current_pointer_valid(), true);
        intcode.next()?;

        assert_eq!(intcode.get_instruction_vector()?, [30]);
        assert_eq!(intcode.is_current_pointer_valid(), false);
        intcode.next()?;

        assert_eq!(intcode.get_instruction_vector()?, [40]);
        assert_eq!(intcode.is_current_pointer_valid(), false);
        intcode.next()?;

        assert_eq!(intcode.get_instruction_vector()?, [50]);
        assert_eq!(intcode.is_current_pointer_valid(), false);

        // now that the intcode is fully parsed, it should return an error
        assert_eq!(intcode.next().is_err(), true);

        Ok(())
    }
}