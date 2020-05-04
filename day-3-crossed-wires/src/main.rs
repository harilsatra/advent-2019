use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::collections::HashSet;

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

    let mut points1: HashSet<String> = HashSet::new();
    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;

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

        if dir == "L" || dir == "R" {
            for n in start_x..updated_x {
                let key = format!("({},{})",n,start_y);
                points1.insert(key);
            }
            start_x = updated_x;
        } else {
            for n in start_y..updated_y {
                let key = format!("({},{})",start_x,n);
                points1.insert(key);
            }
            start_y = updated_y;
        }
    }

    start_x = 0;
    start_y = 0;

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

        if dir == "L" || dir == "R" {
            for n in start_x..updated_x {
                let key = format!("({},{})",n,start_y);
                if points1.contains(&key) {
                    let manhattan_distance: i32 = find_manhattan_distance(n, start_y, 0 , 0);

                    if manhattan_distance < result {
                        result = manhattan_distance;
                    }
                }
            }
            start_x = updated_x;
        } else {
            for n in start_y..updated_y {
                let key = format!("({},{})",start_x,n);
                if points1.contains(&key) {
                    let manhattan_distance: i32 = find_manhattan_distance(start_x, n, 0 , 0);

                    if manhattan_distance < result {
                        result = manhattan_distance;
                    }
                }
            }
            start_y = updated_y;
        }
    }

    println!("{}", result);

    Ok(())
}

fn find_manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let x: i32 = x1 - x2;
    let y: i32 = y1 - y2;

    let manhattan_distance: i32 = x.abs() + y.abs();
    
    return manhattan_distance;
}