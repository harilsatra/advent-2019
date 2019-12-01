use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt"; // File should be in root directory

    let input = File::open(path)?;
    let buffered = BufReader::new(input); // BufReader is optimal

    let mut total_fuel: u64 = 0; // If you don't provide mut it is immutable by default

    for line in buffered.lines() {
        let number_string = line?;
        let number_int = number_string.parse::<u64>().unwrap();

        let mut fuel_for_fuel = number_int;

        loop { // loop is a dedicated keyword for while true (infinite loop)
            fuel_for_fuel = fuel_for_fuel/3;

            if fuel_for_fuel <= 2 {
                break;
            }

            fuel_for_fuel = fuel_for_fuel - 2; 
            total_fuel += fuel_for_fuel;
        }
    }

    println!("{}", total_fuel);

    Ok(())
}

// Correct Output (Part 1): 3286680

// Correct Output (Part 2): 4927158