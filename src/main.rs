use std::error;

use day1;

fn main() -> Result<(), Box<dyn error::Error>>  {
    println!("Hello, world!");

    // day 1 solutions
    let total_initial_fuel = day1::part1::main()?;
    println!("The total fuel requirement for the rocket is {}", total_initial_fuel);
    
    let total_fuel = day1::part2::main()?;
    println!("The total fuel requirement (considering the inner fuel requirements) for the rocket is {}", total_fuel);

    // day 2 solutions
    Ok(())
}
