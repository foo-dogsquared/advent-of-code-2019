pub mod part1 {
    use std::fs;
    use crate::Result;
    
    pub fn main () -> Result<i64> {
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
}

pub mod part2 {
    use std::fs;
    use crate::Result;

    pub fn main () -> Result<i64> {
        let mut input = fs::read_to_string("inputs/day1.txt")?;
        let mut total_fuel = 0;

        for line in input.lines() {
            // initial fuel amount
            let mut fuel: i64 = rocket_fuel_equation(line.parse::<i64>()?);
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
            total_fuel += total_fuel_requirement;
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
}