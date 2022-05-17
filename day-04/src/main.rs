use std::collections::HashSet;
use std::io::{Read, Result};

struct Board {
    tiles: Vec<u8>,
}

// Not FromStr, because the input is expected to be correct, thus there is no need to return a
// Result type
impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let mut tiles = Vec::with_capacity(25);

        for line in input.lines() {
            let nums = line.split_whitespace().map(|s| s.parse::<u8>().unwrap());
            tiles.extend(nums);
        }

        assert_eq!(tiles.len(), 25);

        Board { tiles }
    }
}

impl Board {
    fn is_won(&self, drawn_numbers: &HashSet<u8>) -> bool {
        // row-wise
        for row in 0..5 {
            // Tiles of a row lie next to each other in memory. We can take a slice of the row and
            // iterate over it
            if self.tiles[(5 * row)..(5 * row + 5)]
                .iter()
                .all(|t| drawn_numbers.contains(t))
            {
                return true;
            }
        }

        // col-wise
        for col in 0..5 {
            if (0..5)
                // Map rows of the column to the numbers in the column
                .map(|row| self.tiles[5 * row + col])
                .all(|t| drawn_numbers.contains(&t))
            {
                return true;
            }
        }

        // Same in a bit shorter form
        // if (0..5).any(|col| {
        //     (0..5)
        //         .map(|row| self.tiles[5 * row + col])
        //         .all(|t| drawn_numbers.contains(&t))
        // }) {
        //     return true;
        // }

        false
    }

    fn unmarked_sum(&self, drawn_numbers: &HashSet<u8>) -> i32 {
        self.tiles
        .iter()
        .filter(|t| !drawn_numbers.contains(t))
        .fold(0, |acc, t| acc + *t as i32)
    }
}

fn read_input() -> Result<(Vec<u8>, Vec<Board>)> {
    let mut input_file = std::fs::File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let blocks: Vec<&str> = input.split("\n\n").collect();
    let nums: Vec<u8> = blocks[0].split(',').map(|s| s.parse().unwrap()).collect();
    let boards: Vec<Board> = blocks[1..].iter().map(|&b| b.into()).collect();

    // Could even be written without collecting the blocks into a vector to avoid heap allocation,
    // even though we only store string slices which are references.
    // let mut blocks_iter = input.split("\n\n");
    // let nums: Vec<u8> = blocks_iter
    //     .next()
    //     .unwrap()
    //     .split(',')
    //     .map(|s| s.parse().unwrap())
    //     .collect();
    // let boards: Vec<Board> = blocks_iter.map(|b| b.into()).collect();

    Ok((nums, boards))
}

fn task_1(mut nums: Vec<u8>, boards: &Vec<Board>) {
    // So we can pop from the end with O(1); popping from the front is O(n)
    nums.reverse();
    let mut drawn_numbers = HashSet::<u8>::with_capacity(nums.len());

    let (winner_board, last_num) = loop {
        // Unwrapping, because game has to be won when all numbers were drawn.
        let num = nums.pop().unwrap();
        drawn_numbers.insert(num);

        if let Some(board) = boards.iter().find(|b| b.is_won(&drawn_numbers)) {
            break (board, num);
        }
    };

    let result = winner_board.unmarked_sum(&drawn_numbers) * last_num as i32;
    println!("Task 1: {}", result);
}

fn task_2(mut nums: Vec<u8>, boards: &Vec<Board>) {
    // Popping from end with O(1)
    nums.reverse();

    // Get vector over board references
    let mut boards: Vec<&Board> = boards.iter().collect();
    let mut drawn_numbers = HashSet::<u8>::with_capacity(nums.len());

    let mut last_num = *nums.last().unwrap();
    let mut last_winning_board = boards[0];

    while !boards.is_empty() {
        // Assuming that there is only one board left at the end
        last_winning_board = boards[0];
        // Assuming that every board has to be won when all numbers were drawn
        last_num = nums.pop().unwrap();
        drawn_numbers.insert(last_num);

        boards = boards
            .into_iter()
            .filter(|b| !b.is_won(&drawn_numbers))
            .collect();
    }

    let result = last_winning_board.unmarked_sum(&drawn_numbers) * last_num as i32;
    println!("Task 2: {}", result);
}

fn main() {
    let (nums, boards) = read_input().expect("error reading input");
    task_1(nums.clone(), &boards);
    task_2(nums, &boards);
}
