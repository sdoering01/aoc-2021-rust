use std::io::{Read, Result};

type Input = ();

fn read_input() -> Result<Input> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    // Parse input

    Ok(())
}

fn task_1(input: Input) {
    let solution = "todo";
    println!("Task 1: {}", solution);
}

fn task_2(input: Input) {
    let solution = "todo";
    println!("Task 2: {}", solution);
}

fn main() {
    let input = read_input().expect("error reading input");
    task_1(input);
    task_2(input);
}
