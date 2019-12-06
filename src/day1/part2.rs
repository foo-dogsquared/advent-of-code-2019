use std::fs;
use crate::Result;

pub fn main () -> Result<i64> {
    let mut input = fs::read_to_string("inputs/day1.txt")?;
    let mut total_fuel = 0;

    for line in input.lines() {
        // initial fuel amount
        let fuel: i64 = rocket_fuel_equation_with_inner_fuel(line.parse::<i64>()?);
        total_fuel += fuel;
    }

    Ok(total_fuel)
}

pub fn rocket_fuel_equation (mass: i64) -> i64 {
    (mass / 3) - 2
}

/// Calculates the rocket fuel needed considering the fuel itself. 
/// 
/// Even though this could easily be a recursive function, I try to make it possible only with loops. 
pub fn rocket_fuel_equation_with_inner_fuel (mass: i64) -> i64 {
    let mut fuel = rocket_fuel_equation(mass);
    let mut fuel_vector: Vec<i64> = vec![fuel];

    // calculating the fuel requirement of further fuel
    loop {
        let fuel_requirement = rocket_fuel_equation(fuel);
        if fuel_requirement > 0 {
            fuel = fuel_requirement;
            fuel_vector.push(fuel);
        }  else {
            break;
        }
    }

    let total_fuel_requirement: i64 = fuel_vector.iter().sum();

    total_fuel_requirement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_example() {
        assert_eq!(rocket_fuel_equation_with_inner_fuel(12), 2);
        assert_eq!(rocket_fuel_equation_with_inner_fuel(14), 2);
        assert_eq!(rocket_fuel_equation_with_inner_fuel(1969), 966);
        assert_eq!(rocket_fuel_equation_with_inner_fuel(100756), 50346);
    }
}