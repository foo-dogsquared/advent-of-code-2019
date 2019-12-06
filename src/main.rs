use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    ValueError, 
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ValueError => write!(f, "Value given is not valid."), 
        }
    }
}

pub mod day1;

fn main() -> Result<()>  {
    println!("Hello, world!");

    // day 1 solutions
    let total_initial_fuel = day1::part1::main()?;
    println!("The total fuel requirement for the rocket is {}", total_initial_fuel);
    
    let total_fuel = day1::part2::main()?;
    println!("The total fuel requirement (considering the inner fuel requirements) for the rocket is {}", total_fuel);

    // day 2 solutions    
    Ok(())
}
