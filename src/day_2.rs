use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

pub fn group_by_letter_duplicates(input: Vec<String>) -> HashMap<usize, usize> {
    let mut results = HashMap::new();

    for s in input.iter() {
        for val in s
            .chars()
            .sorted()
            .group_by(|c| c.clone())
            .into_iter()
            .map(|(_key, group)| group.count())
            .sorted()
            .dedup()
        {
            results.insert(val, results.get(&val).unwrap_or(&0) + 1);
        }
    }

    results
}

pub fn checksum(input: Vec<String>) -> usize {
    let m = group_by_letter_duplicates(input);

    m.get(&2).unwrap_or(&0) * m.get(&3).unwrap_or(&0)
}

pub fn checksum_from_file(filename: &str) -> usize {
    checksum(read_file(filename))
}

fn read_file(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("File not found");
    let r = BufReader::new(f);

    r.lines().map(|e| e.expect("error reading file")).collect()
}

pub fn find_correct_letters(input: Vec<String>) -> String {
    for i in input.iter() {
        for j in input.iter() {
            if i == j {
                break;
            }

            let mut rest = Vec::new();
            for (left, right) in itertools::zip(i.chars(), j.chars()) {
                if left == right {
                    rest.push(left);
                }
            }

            if rest.len() == i.len() - 1 {
                return rest.iter().collect();
            }
        }
    }

    "".to_string()
}

pub fn find_correct_letters_with_filename(filename: &str) -> String {
    find_correct_letters(read_file(filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_letter_duplicates() {
        let results = group_by_letter_duplicates(
            vec![
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        assert_eq!(results.get(&2), Some(&4));
        assert_eq!(results.get(&3), Some(&3));
    }

    #[test]
    fn test_checksum() {
        let result = checksum(
            vec![
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        assert_eq!(result, 12);
    }

    #[test]
    fn test_day_2_part_1() {
        let result = checksum_from_file("files/day_2.txt");

        assert_eq!(5704, result);
    }

    #[test]
    fn test_difference() {
        let result = find_correct_letters(
            vec![
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        assert_eq!(result, "fgij");
    }

    #[test]
    fn test_day_2_part_2() {
        let result = find_correct_letters_with_filename("files/day_2.txt");
        assert_eq!("umdryabviapkozistwcnihjqx", result);
    }
}
