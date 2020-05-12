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
    println!("Input: {}-{}", a, b);

    let mut count1: i32 = 0;
    let mut count2: i32 = 0;
    for i in a..b {
        if validate_part1(i) {
            count1 = count1 + 1
        }

        if validate_part2(i) {
            count2 = count2 + 1
        }
    }

    println!("Part1: {}", count1);
    println!("Part2: {}", count2);
    Ok(())
}

fn validate_part1(num: i32) -> bool {
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

fn validate_part2(num: i32) -> bool {
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
    let mut similar_count: i32 = 1;
    let mut has_similar: bool = false;
    for (i, digit) in digits.iter().enumerate() {
        if i == 0 {
            prev = *digit;
        } else {
            if prev == *digit  {
                similar_count = similar_count + 1;
            } else if prev > *digit {
                if similar_count == 2 {
                    has_similar = true;
                }
                similar_count = 1
            } else {
                return false
            }
            prev = *digit;
        }
    }

    if similar_count == 2 {
        has_similar = true;
    }

    if !has_similar {
        return false
    }

    return true;
}

// Most unit tests go into a tests mod with the #[cfg(test)] attribute. Test functions are marked with the #[test] attribute.
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate_part1() {
        assert_eq!(validate_part1(111111), true);
        assert_eq!(validate_part1(223450), false);
        assert_eq!(validate_part1(123789), false);
        assert_eq!(validate_part1(112233), true);
        assert_eq!(validate_part1(123444), true);
        assert_eq!(validate_part1(111122), true);
        assert_eq!(validate_part1(223333), true);
        assert_eq!(validate_part1(223334), true);
    }

    #[test]
    fn test_validate_part2() {
        assert_eq!(validate_part2(111111), false);
        assert_eq!(validate_part2(223450), false);
        assert_eq!(validate_part2(123789), false);
        assert_eq!(validate_part2(112233), true);
        assert_eq!(validate_part2(123444), false);
        assert_eq!(validate_part2(111122), true);
        assert_eq!(validate_part2(223333), true);
        assert_eq!(validate_part2(223334), true);
    }
}