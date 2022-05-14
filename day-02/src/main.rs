use std::io::{Read, Result};

fn read_input() -> Result<Vec<(String, i32)>> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let mut commands: Vec<(String, i32)> = vec![];
    for line in input.lines() {
        let line_items: Vec<&str> = line.splitn(2, ' ').collect();
        commands.push((line_items[0].to_owned(), line_items[1].parse().unwrap()));
    }
    Ok(commands)
}

fn task_1(commands: &Vec<(String, i32)>) {
    // (horizontal pos, depth)
    let mut pos = (0, 0);

    for (dir, dist) in commands {
        match dir.as_str() {
            "forward" => pos.0 += dist,
            "up" => pos.1 -= dist,
            "down" => pos.1 += dist,
            _ => panic!("unknown direction: {}", dir)
        }
    }

    // Alternative using fold
    // let pos = commands
    //     .iter()
    //     .fold((0, 0), |(x, y), (dir, dist)| match dir.as_str() {
    //         "forward" => (x + dist, y),
    //         "up" => (x, y - dist),
    //         "down" => (x, y + dist),
    //         _ => panic!("unknown direction: {}", dir),
    //     });

    let result = pos.0 * pos.1;
    println!("Task 1: {}", result);
}

fn task_2(commands: &Vec<(String, i32)>) {
    // (horizontal pos, depth)
    let mut pos = (0, 0);
    let mut aim = 0;

    for (dir, dist) in commands {
        match dir.as_str() {
            "forward" => {
                pos.0 += dist;
                pos.1 += dist * aim;
            },
            "up" => aim -= dist,
            "down" => aim += dist,
            _ => panic!("unknown direction: {}", dir)
        }
    }

    let result = pos.0 * pos.1;
    println!("Task 2: {}", result);
}

fn main() {
    let commands = read_input().expect("error reading input");
    task_1(&commands);
    task_2(&commands);
}
