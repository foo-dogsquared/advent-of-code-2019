use std::fs;
use std::error;

pub fn main () -> Result<i64, Box<dyn error::Error>> {
    let mut input = fs::read_to_string("inputs/day1.txt")?;
    let mut total_fuel = 0;

    for line in input.lines() {
        total_fuel += rocket_fuel_equation(line.parse::<i64>()?);
    }

    Ok(total_fuel)
}

pub fn rocket_fuel_equation (mass: i64) -> i64 {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_example() {
        assert_eq!(rocket_fuel_equation(12), 2);
        assert_eq!(rocket_fuel_equation(14), 2);
        assert_eq!(rocket_fuel_equation(1969), 654);
        assert_eq!(rocket_fuel_equation(100756), 33583);
    }
}