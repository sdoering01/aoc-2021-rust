use std::collections::HashMap;
use std::io::{Read, Result};

#[derive(Debug, Clone)]
struct DigitDisplay {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl DigitDisplay {
    fn take_pattern_by_segment_count(&mut self, digit_count: usize) -> String {
        let (idx, pattern) = self
            .patterns
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, p)| p.len() == digit_count)
            .unwrap();
        self.patterns.swap_remove(idx);
        pattern
    }

    fn get_output_value(mut self) -> i32 {
        let one_pattern = self.take_pattern_by_segment_count(2);
        let four_pattern = self.take_pattern_by_segment_count(4);
        let seven_pattern = self.take_pattern_by_segment_count(3);
        let eight_pattern = self.take_pattern_by_segment_count(7);

        let zero_six_nine_patterns = [
            self.take_pattern_by_segment_count(6),
            self.take_pattern_by_segment_count(6),
            self.take_pattern_by_segment_count(6),
        ];
        let mut zero_pattern = String::new();
        let mut six_pattern = String::new();
        let mut nine_pattern = String::new();
        for pattern in zero_six_nine_patterns {
            match (
                is_segment_superset(&pattern, &four_pattern),
                is_segment_superset(&pattern, &seven_pattern),
            ) {
                (true, true) => nine_pattern = pattern,
                (false, true) => zero_pattern = pattern,
                (false, false) => six_pattern = pattern,
                _ => unreachable!(),
            }
        }

        let two_three_five_patterns = [
            self.take_pattern_by_segment_count(5),
            self.take_pattern_by_segment_count(5),
            self.take_pattern_by_segment_count(5),
        ];
        let mut two_pattern = String::new();
        let mut three_pattern = String::new();
        let mut five_pattern = String::new();
        for pattern in two_three_five_patterns {
            if is_segment_superset(&six_pattern, &pattern) {
                five_pattern = pattern;
            } else if is_segment_superset(&pattern, &seven_pattern) {
                three_pattern = pattern;
            } else {
                two_pattern = pattern;
            }
        }

        let pattern_values = HashMap::from([
            (zero_pattern, 0),
            (one_pattern, 1),
            (two_pattern, 2),
            (three_pattern, 3),
            (four_pattern, 4),
            (five_pattern, 5),
            (six_pattern, 6),
            (seven_pattern, 7),
            (eight_pattern, 8),
            (nine_pattern, 9),
        ]);

        self.output
            .into_iter()
            .map(|pattern| pattern_values.get(&pattern).unwrap())
            .fold(0, |acc, val| 10 * acc + val)
    }
}

fn read_input() -> Result<Vec<DigitDisplay>> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    // acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    let digit_displays = input
        .lines()
        .map(|line| {
            let mut parts_iter = line.split(" | ");

            let patterns_part = parts_iter.next().unwrap();
            let patterns = patterns_part
                .split(' ')
                .map(|pattern| {
                    let mut chars: Vec<char> = pattern.chars().collect();
                    chars.sort();
                    chars.into_iter().collect()
                })
                .collect();

            let output_part = parts_iter.next().unwrap();
            let output = output_part
                .split(' ')
                .map(|pattern| {
                    let mut chars: Vec<char> = pattern.chars().collect();
                    chars.sort();
                    chars.into_iter().collect()
                })
                .collect();

            DigitDisplay { patterns, output }
        })
        .collect();

    Ok(digit_displays)
}

/// digit_1 and digit_2 have to be sorted
fn is_segment_superset(digit_1: &str, digit_2: &str) -> bool {
    let mut digit_1_iter = digit_1.chars();

    for c2 in digit_2.chars() {
        loop {
            match digit_1_iter.next() {
                Some(c1) => {
                    if c1 == c2 {
                        break;
                    }
                }
                None => return false,
            }
        }
    }

    true
}

#[test]
fn test_is_segment_superset() {
    assert!(is_segment_superset("abc", "ab"));
    assert!(!is_segment_superset("ab", "abc"));
    assert!(is_segment_superset("ab", "ab"));
    assert!(is_segment_superset("abde", "be"));
    assert!(!is_segment_superset("abde", "abdef"));
}

fn task_1(displays: Vec<DigitDisplay>) {
    let solution = displays
        .into_iter()
        .flat_map(|display| display.output)
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count();
    println!("Task 1: {}", solution);
}

fn task_2(displays: Vec<DigitDisplay>) {
    let solution: i32 = displays
        .into_iter()
        .map(|display| display.get_output_value())
        .sum();
    println!("Task 2: {}", solution);
}

fn main() {
    let input = read_input().expect("error reading input");
    task_1(input.clone());
    task_2(input);
}
