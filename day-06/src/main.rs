use std::io::{Read, Result};

type Population = [u64; 9];

fn read_input() -> Result<Population> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut population: Population = [0; 9];
    input.trim_end_matches('\n').split(',').for_each(|num| {
        let idx = num.parse::<usize>().unwrap();
        population[idx] += 1;
    });

    Ok(population)
}

fn simulate_day(population: Population) -> Population {
    let mut new_population = [0; 9];
    new_population[6] = population[0];
    new_population[8] = population[0];
    for i in 1..9 {
        new_population[i - 1] += population[i];
    }
    new_population
}

fn simulate(mut population: Population, days: u32) -> Population {
    for _ in 0..days {
        population = simulate_day(population);
    }
    population
}

fn task_1(population: Population) {
    let total: u64 = simulate(population, 80).iter().sum();
    println!("Task 1: {}", total);
}

fn task_2(population: Population) {
    let total: u64 = simulate(population, 256).iter().sum();
    println!("Task 1: {}", total);
}

fn main() {
    let population = read_input().expect("error reading input");
    task_1(population);
    task_2(population);
}
