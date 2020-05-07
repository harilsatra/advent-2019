use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut lines: Vec<String> = vec![];

    // read the input
    for line in buffered.lines() {
        lines.push(line?);
    }

    if lines.len() != 1 {
        return Err(Error::new(
            ErrorKind::Other,
            "Invalid input",
        ));
    }

    let range:Vec<&str> = lines[0].split("-").collect();
    let a = range[0].parse::<i32>().unwrap();
    let b = range[1].parse::<i32>().unwrap();
    println!("{}, {}", a, b);

    let mut count: i32 = 0;
    for i in a..b {
        if validate(i) {
            count = count + 1
        }
    }

    println!("{}", count);
    Ok(())
}

fn validate(num: i32) -> bool {
    let mut temp = num;

    let mut digits = Vec::new();
    while temp != 0 {
        digits.push(temp % 10);
        temp = temp / 10;
    }

    if digits.len() != 6 {
        return false;
    }

    let mut prev: i32 = 0;
    let mut similar_count: i32 = 0;
    for (i, digit) in digits.iter().enumerate() {
        if i == 0 {
            prev = *digit;
        } else {
            if prev < *digit {
                return false
            } else if prev == *digit {
                similar_count = similar_count + 1;
            }
            prev = *digit;
        }
    }

    if similar_count == 0 {
        return false
    }

    return true;
}
