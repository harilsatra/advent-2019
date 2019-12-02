use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input); // BufReader is optimal

    let mut program_string = "".to_string(); // line is of type String and just initializing it to "" makes it of type &str
    let mut no_of_lines = 0;

    for line in buffered.lines() {
        program_string = line?;
        no_of_lines += 1;
    }

    if no_of_lines != 1 {
        return Err(Error::new(
            ErrorKind::Other,
            "Invalid input",
        ));
    }
    
    let mut program: Vec<i32> = program_string.split(",").map(|s| i32::from_str(s).unwrap()).collect();

    for i in (0..program.len()).step_by(4) {
        let pos1 = program[i+1] as usize;
        let pos2 = program[i+2] as usize;
        let pos3 = program[i+3] as usize;

        if program[i] == 1 {
            program[pos3] = program[pos1] + program[pos2];
        } else if program[i] == 2 {
            program[pos3] = program[pos1] * program[pos2];
        } else if program[i] == 99 {
            break;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Something went wrong",
            ));
        }
     }

    println!("{}", program[0]); // std::vec::Vec<&str>` cannot be formatted with the default formatter

    Ok(())
}
