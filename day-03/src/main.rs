use std::io::{Read, Result};

fn read_input() -> Result<Vec<Vec<u8>>> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut entries = vec![];
    for line in input.lines() {
        let bits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        entries.push(bits);
    }

    Ok(entries)
}

fn task_1(entries: &Vec<Vec<u8>>) {
    let mut ones: Vec<i32> = vec![0; entries[0].len()];

    for entry in entries {
        for (pos, bit) in entry.iter().enumerate() {
            ones[pos] += *bit as i32;
        }
    }

    let num_half_entries = entries.len() as i32 / 2;
    let gamma = ones
        .iter()
        // Map count of ones to bits in gamma rate
        .map(|num_ones| if *num_ones > num_half_entries { 1 } else { 0 })
        // Convert bits to int
        .fold(0, |acc, bit| (acc << 1) + bit);

    // epsilon has all bits (excepct leading zeros) of gamma flipped. When added, epsilon and
    // gamma will produce a binary number with only ones and the same amount of bits as the input
    // entries. The sum is one less than the binary number starting with a one, followed by the same
    // amount of zeros (this number is `1 << ones.len()`).
    let epsilon = (1 << ones.len()) - gamma - 1;

    let result = gamma * epsilon;
    println!("Task 1: {}", result);
}

fn task_2(entries: &Vec<Vec<u8>>) {
    let oxygen_rate = get_rate(entries, |num_zeros, num_ones| num_ones >= num_zeros);
    let co2_rate = get_rate(entries, |num_zeros, num_ones| num_ones < num_zeros);

    let result = oxygen_rate * co2_rate;
    println!("Task 2: {}", result);
}

/// Calculates the rate of one entry. The entry is picked by filtering all entries until one is
/// left.
///
/// In each filter step we look at the same bit of all candidates, starting at the first bit
/// and incrementing the bit position after each filter step. We then count the amount of ones and
/// zeros of all entries in that bit.
///
/// To decide whether to keep the candidates that have a one or a zero in that bit, we use the
/// `should_pick_ones` function. It gets passed the amount of zeros and ones and returns whether
/// the candidates with ones should be kept, otherwise the candidates with zeros are kept. This way
/// we can reuse the function to calculate both the oxygen and the co2 rate.
fn get_rate<F>(entries: &Vec<Vec<u8>>, should_pick_ones: F) -> i32
where
    F: Fn(usize, usize) -> bool
{
    let mut candidates: Vec<usize> = (0..entries.len()).into_iter().collect();
    let mut offset = 0;

    while candidates.len() > 1 && offset < entries[0].len() {
        let mut zero_candidates = Vec::new();
        let mut one_candidates = Vec::new();

        for idx in &candidates {
            if entries[*idx][offset] == 0 {
                zero_candidates.push(*idx);
            } else {
                one_candidates.push(*idx);
            }
        }

        if should_pick_ones(zero_candidates.len(), one_candidates.len()) {
            candidates = one_candidates;
        } else {
            candidates = zero_candidates;
        }

        offset += 1;
    }

    assert_eq!(candidates.len(), 1, "filter didn't yield exactly one entry");
    let entry = &entries[candidates[0]];
    let rate: i32 = entry.iter().fold(0, |acc, bit| (acc << 1) + *bit as i32);

    rate
}

fn main() {
    let entries = read_input().expect("error reading input");
    task_1(&entries);
    task_2(&entries);
}
