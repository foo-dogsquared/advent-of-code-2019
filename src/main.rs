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
pub mod day2;

fn print_day_header(day: i64) {
    println!("\n\n-------");
    println!("Day {} Solutions:", day);
    println!("-------\n");
}

fn main() -> Result<()>  {
    
    // day 1 solutions
    print_day_header(1);
    let total_initial_fuel = day1::part1::main()?;
    println!("The total fuel requirement for the rocket is {}", total_initial_fuel);
    
    let total_fuel = day1::part2::main()?;
    println!("The total fuel requirement (considering the inner fuel requirements) for the rocket is {}", total_fuel);

    // day 2 solutions
    print_day_header(2);
    let intcode_result = day2::part1::main()?;
    println!("The resulting value for input 0 is {}.", intcode_result[0]); 

    let intcode_result_for_part2 = day2::part2::main()?;
    println!("The noun and the verb to make up '19690720' is {} and {}, respectively. The puzzle answer is {}.", intcode_result_for_part2[1], intcode_result_for_part2[2], intcode_result_for_part2[1] + intcode_result_for_part2[2]); 
    Ok(())
}
