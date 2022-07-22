use std::io::{Read, Result};

type Point = (i32, i32);
type PointPair = (Point, Point);

fn read_input() -> Result<Vec<PointPair>> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let line_points: Vec<PointPair> = input
        .lines()
        // Input lines look like this: `x1,y1 -> x2,y2`
        .map(|input_line| {
            let point_pair: Vec<Point> = input_line
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

fn line_iter(&((x1, y1), (x2, y2)): &PointPair) -> impl Iterator<Item = Point> {
    // We have to use a trait object (Box<dyn ...>), because the specific types of the returned
    // iterators differ depending on the input.
    fn coord_iter(c1: i32, c2: i32) -> Box<dyn Iterator<Item = i32>> {
        use std::cmp::Ordering;
        match c1.cmp(&c2) {
            Ordering::Less => Box::new(c1..=c2),
            // Ranges can't count down, so we construct it the other way around and reverse it.
            Ordering::Greater => Box::new((c2..=c1).rev()),
            // Assuming that the other coordinate's points aren't equal too. The other coord
            // iterator will limit the elements of the zipped line iterator.
            Ordering::Equal => Box::new(std::iter::repeat(c1)),
        }
    }

    let x_iter = coord_iter(x1, x2);
    let y_iter = coord_iter(y1, y2);

    x_iter.zip(y_iter)
}

fn get_overlapping_points(line_points: &[PointPair], include_diagonals: bool) -> usize {
    // use std::collections::HashMap;
    //
    // let mut point_map = HashMap::<Point, i32>::new();
    // // Start with a higher capacity (which is an educated guess) to avoid unnecessary rehasing
    // // let mut point_map = HashMap::<Point, i32>::with_capacity(300 * line_points.len());
    // line_points
    //     .iter()
    //     // Only evaluate horizontal and vertical lines, when `include_diagonals` is false.
    //     .filter(|((x1, y1), (x2, y2))| include_diagonals || x1 == x2 || y1 == y2)
    //     .for_each(|point_pair| {
    //         for p in get_line_iter(point_pair) {
    //             let num_points = point_map.entry(p).or_insert(0);
    //             *num_points += 1;
    //         }
    //     });
    //
    // let num_overlapping_points = point_map
    //     .values()
    //     .filter(|num_points| **num_points > 1)
    //     .count();

    // ~1.5x faster but probably uses more memory, because many points won't have any line on it
    let mut map = [0; 1_000 * 1_000];
    line_points
        .iter()
        // Only evaluate horizontal and vertical lines, when `include_diagonals` is false.
        .filter(|((x1, y1), (x2, y2))| include_diagonals || x1 == x2 || y1 == y2)
        // Even more declarative
        .flat_map(line_iter)
        .for_each(|(x, y)| {
            map[(x + y * 1_000) as usize] += 1;
        });
        // .for_each(|point_pair| {
        //     for (x, y) in line_iter(point_pair) {
        //         let idx = (1000 * y + x) as usize;
        //         map[idx] += 1;
        //     }
        // });

    let num_overlapping_points = map.iter().filter(|num_points| **num_points > 1).count();

    num_overlapping_points
}

fn task_1(line_points: &[PointPair]) {
    let result = get_overlapping_points(line_points, false);
    println!("Task 1: {}", result);
}

fn task_2(line_points: &[PointPair]) {
    let result = get_overlapping_points(line_points, true);
    println!("Task 2: {}", result);
}

fn main() {
    let line_points = read_input().expect("error reading input");
    task_1(&line_points);
    task_2(&line_points);
}
