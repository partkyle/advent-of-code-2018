use std::io::prelude::*;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;


pub fn parse_frequency(s: &str) -> Result<i32, String> {
    i32::from_str_radix(s, 10).map_err(|e| e.to_string())
}

pub fn calculate_frequency(file: &str) -> Result<i32, String> {
    let f = File::open(file).expect("File not found");
    let r = BufReader::new(f);

    r.lines().map(|line| match line {
        Ok(s) => parse_frequency(&s[..]),
        Err(err) => Err(err.to_string())
    }).sum()
}

pub fn calibrate(file: &str) -> Result<i32, String> {
    let mut frequencies = HashSet::new();
    let mut freq = 0;

    // use a buffered reader to get access to line-by-line reading
    let f = File::open(file).expect("File not found");
    let r = BufReader::new(f);

    // collect the lines into a collection that doesn't require matching
    // over Result types
    let lines: Result<Vec<_>, _> = r.lines().collect();
    let lines = lines.map_err(|e| e.to_string())?;

    // loop continually until a frequency is found
    // Note: if a frequency is not found, it will continually loop
    // causing a deadlock
    loop {
        for line in lines.iter() {
            let num = parse_frequency(&line[..])?;
            freq += num;

            if frequencies.contains(&freq) {
                return Ok(freq);
            }

            frequencies.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frequency() {
        assert_eq!(
            Ok(1),
            parse_frequency("+1")
        );

        assert_eq!(
            Ok(-19),
            parse_frequency("-19")
        );

        assert_eq!(
            Err("invalid digit found in string".to_string()),
            parse_frequency("boof")
        );
    }

    #[test]
    pub fn test_day_1_part_1() {
        assert_eq!(
            580,
            calculate_frequency("files/day_1.txt").unwrap()
        );
    }

    #[test]
    pub fn test_day_1_part_2() {
        assert_eq!(
            81972,
            calibrate("files/day_1.txt").unwrap()
        );
    }
}
