use std::io::prelude::*;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

pub fn handle_frequency_line(line: Result<String, std::io::Error>) -> Result<i32, String> {
    let line = line.map_err(|e| e.to_string())?;
    i32::from_str_radix(&line[..], 10).map_err(|e| e.to_string())
}

pub fn calculate_frequency(file: &str) -> Result<i32, String> {
    let f = File::open(file).expect("File not found");
    let r = BufReader::new(f);

    r.lines().map(handle_frequency_line).sum()
}

pub fn calibrate(file: &str) -> Result<i32, String> {
    let mut frequencies = HashSet::new();
    let mut freq = 0;
    loop {
        let f = File::open(file).expect("File not found");
        let r = BufReader::new(f);

        for line in r.lines() {
            let num = handle_frequency_line(line)?;
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
    pub fn test_day_1() {
        assert_eq!(
            Ok(580),
            calculate_frequency("files/day_1.txt")
        );
    }

    #[test]
    pub fn test_day_2() {
        assert_eq!(
            Ok(81972),
            calibrate("files/day_1.txt")
        );
    }
}
