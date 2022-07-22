use std::io::{Read, Result};

type Positions = Vec<u32>;

fn read_input() -> Result<Positions> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let positions = input
        .trim_end_matches('\n')
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    Ok(positions)
}

fn calc_optimal_cost(positions: Positions, cost_fn: impl Fn(u32) -> u32) -> u32 {
    let min_pos = positions.iter().min().copied().unwrap();
    let max_pos = positions.iter().max().copied().unwrap();

    let optimal_cost: u32 = (min_pos..=max_pos)
        .map(|target| {
            positions
                .iter()
                .map(|&pos| cost_fn(u32::abs_diff(target, pos)))
                .sum()
        })
        .min()
        .unwrap();

    optimal_cost
}

fn task_1(positions: Positions) {
    let optimal_cost = calc_optimal_cost(positions, |d| d);
    println!("Task 1: {}", optimal_cost);
}

fn task_2(positions: Positions) {
    let optimal_cost = calc_optimal_cost(positions, |d| ((d + 1) * d) / 2);
    println!("Task 2: {}", optimal_cost);
}

fn main() {
    let input = read_input().expect("error reading input");
    task_1(input.clone());
    task_2(input);
}
