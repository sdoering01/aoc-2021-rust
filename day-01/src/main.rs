use std::io::{Read, Result};

fn get_input() -> Result<Vec<i32>> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let nums: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    Ok(nums)
}

fn task_1(input: Vec<i32>) {
    // let mut increases = 0;
    // let mut last = input[0];
    // for cur in &input[1..] {
    //     if *cur > last {
    //         increases += 1;
    //     }
    //     last = *cur;
    // }

    // Alternative with zip
    let diffs = std::iter::zip(&input, &input[1..]);
    let increases = diffs.filter(|(a, b)| b > a).count();

    println!("Task 1: {}", increases);
}

fn task_2(input: Vec<i32>) {
    // Naive way: Simply compare the sums of the windows.
    // let mut increases = 0;
    // let mut a = input[0];
    // let mut b = input[1];
    // let mut c = input[2];
    //
    // for cur in &input[3..] {
    //     if b + c + cur > a + b + c {
    //         increases += 1;
    //     }
    //     a = b;
    //     b = c;
    //     c = *cur;
    // }

    // Smarter way: When we have two windows A and B, window B has a larger sum, when its last
    // value is greater than the first value of A. Because the windows only differ in those two
    // values.
    let window_size = 3;
    let window_diffs = std::iter::zip(&input, &input[window_size..]);

    // Using a for loop
    // let mut increases = 0;
    // for (a, b) in window_diffs {
    //     if b > a {
    //         increases += 1;
    //     }
    // }

    // Alternative: Using iterator functions
    let increases = window_diffs.filter(|(a, b)| b > a).count();

    println!("Task 2: {}", increases);
}

fn main() {
    let input = get_input().expect("error reading input");
    task_1(input.clone());
    task_2(input);
}
