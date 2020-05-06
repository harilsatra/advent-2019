use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut lines: Vec<String> = vec![];

    // read the input
    for line in buffered.lines() {
        lines.push(line?);
    }

    if lines.len() != 2 {
        return Err(Error::new(
            ErrorKind::Other,
            "Invalid input",
        ));
    }

    let mut points1: HashMap<String, i32> = HashMap::new();
    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;
    let mut i: i32 = 0;

    for point in lines[0].split(",") {
        let dir =  &point[..1];
        let dist = &point[1..].parse::<i32>().unwrap();

        let mut updated_x: i32 = 0;
        let mut updated_y: i32 = 0;

        match dir {
            "L" => updated_x = start_x - dist,
            "R" => updated_x = start_x + dist,
            "U" => updated_y = start_y + dist,
            "D" => updated_y = start_y - dist,
            _ => return Err(Error::new(
                ErrorKind::Other,
                "Invalid dir in one of the inputs",
            )),
        }

        if dir == "R" {
            for n in start_x..updated_x {
                let key = format!("({},{})",n,start_y);
                points1.insert(key, i);
                i = i + 1;
            }
            start_x = updated_x;
        } else if dir == "L" {
            let mut n = start_x;
            while n > updated_x {
                let key = format!("({},{})",n,start_y);
                points1.insert(key, i);
                i = i + 1;
                n = n - 1;
            }
            start_x = updated_x;
        } else if dir == "U" {
            for n in start_y..updated_y {
                let key = format!("({},{})",start_x,n);
                points1.insert(key, i);
                i = i + 1;
            }
            start_y = updated_y;
        } else {
            let mut n = start_y;
            while n > updated_y {
                let key = format!("({},{})",start_x,n);
                points1.insert(key, i);
                i = i + 1;
                n = n - 1;
            }
            start_y = updated_y;
        }
    }

    start_x = 0;
    start_y = 0;
    i = 0;

    let mut result: i32 = 2147483647; // Max value of i32

    for point in lines[1].split(",") {
        let dir =  &point[..1];
        let dist = &point[1..].parse::<i32>().unwrap();

        let mut updated_x: i32 = 0;
        let mut updated_y: i32 = 0;

        match dir {
            "L" => updated_x = start_x - dist,
            "R" => updated_x = start_x + dist,
            "U" => updated_y = start_y + dist,
            "D" => updated_y = start_y - dist,
            _ => return Err(Error::new(
                ErrorKind::Other,
                "Invalid dir in one of the inputs",
            )),
        }

        if dir == "R" {
            for n in start_x..updated_x {
                let key = format!("({},{})",n,start_y);

                if key == "(0,0)" {
                    i = i + 1;
                    continue;
                }
                
                let steps = points1.get(&key); 
                if points1.contains_key(&key) && steps.unwrap() + i < result{
                    result = steps.unwrap() + i;
                }
                i = i + 1 
            }
            start_x = updated_x;
        } else if dir == "L" {
            let mut n = start_x;
            while n > updated_x {
                let key = format!("({},{})",n,start_y);

                if key == "(0,0)" {
                    i = i + 1;
                    n = n - 1;
                    continue;
                }
                
                let steps = points1.get(&key); 
                if points1.contains_key(&key) && steps.unwrap() + i < result {
                    result = steps.unwrap() + i;
                }
                i = i + 1;
                n = n - 1;
            }
            start_x = updated_x;
        } else if dir == "U" {
            for n in start_y..updated_y {
                let key = format!("({},{})",start_x,n);

                if key == "(0,0)" {
                    i = i + 1;
                    continue;
                }

                let steps = points1.get(&key); 
                if points1.contains_key(&key) && steps.unwrap() + i < result{
                    result = steps.unwrap() + i;
                }
                i = i + 1 
            }
            start_y = updated_y;
        } else {
            let mut n = start_y;
            while n > updated_y {
                let key = format!("({},{})",start_x,n);

                if key == "(0,0)" {
                    i = i + 1;
                    n = n - 1;
                    continue;
                }

                let steps = points1.get(&key); 
                if points1.contains_key(&key) && steps.unwrap() + i < result{
                    result = steps.unwrap() + i;
                }
                i = i + 1;
                n = n - 1;
            }
            start_y = updated_y;
        }
    }

    println!("{}", result);

    Ok(())
}
