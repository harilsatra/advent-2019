use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut program_string = "".to_string(); // line is of type String and just initializing it to "" makes it of type &str
    let mut no_of_lines = 0;

    // read the input
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
    
    // Iterate over the possible values to find out which one returns the result we are looking for
    // which is 19690720 in our case
    'outer: for x in 0..99 {
        for y in 0..99 {
            let program: Vec<i32> = program_string.split(",").map(|s| i32::from_str(s).unwrap()).collect();
            let output = find_output(program, x, y);
            match output {
                Ok(19690720) => {println!("output: {}, {}", x, y); break 'outer;},
                Ok(_o) => {}, 
                Err(e) => {println!("error: {:?}", e)},
            }
        }
    }

    Ok(())
}

fn find_output(mut program: Vec<i32>, x: i32, y: i32) -> Result<i32, Error> {
    let program_copy = program.as_mut_slice();
    program_copy[1] = x;
    program_copy[2] = y;
    for i in (0..program_copy.len()).step_by(4) {
        let pos1 = program_copy[i+1] as usize;
        let pos2 = program_copy[i+2] as usize;
        let pos3 = program_copy[i+3] as usize;

        if program_copy[i] == 1 {
            program_copy[pos3] = program_copy[pos1] + program_copy[pos2];
        } else if program_copy[i] == 2 {
            program_copy[pos3] = program_copy[pos1] * program_copy[pos2];
        } else if program_copy[i] == 99 {
            break;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Something went wrong",
            ));
        }
     }
    
     Ok(program[0])
}

// Correct Output (Part 1): 3085697

// Correct Output (Part 2): 94, 25
