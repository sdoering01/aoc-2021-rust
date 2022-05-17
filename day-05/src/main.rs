use std::io::{Read, Result};

fn read_input() -> Result<Vec<((i32, i32), (i32, i32))>> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let line_points: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        // Input lines look like this: `x1,y1 -> x2,y2`
        .map(|input_line| {
            let point_pair: Vec<(i32, i32)> = input_line
                .split(" -> ")
                .map(|point_str| {
                    let point: Vec<i32> = point_str
                        .split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    (point[0], point[1])
                })
                .collect();
            (point_pair[0], point_pair[1])
        })
        .collect();

    Ok(line_points)
}

fn main() {
    let line_points = read_input().expect("error reading input");
    println!("{:#?}", line_points);
}
