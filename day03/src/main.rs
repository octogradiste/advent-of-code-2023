use std::{collections::HashSet, fs::read_to_string};

const FILE: &str = "data/input";
// const FILE: &str = "data/example1";

struct Symbol {
    row: usize,
    col: usize,
    symbol: char,
}

fn main() {
    let file = read_to_string(FILE).expect("File not found");
    let lines = file.lines();

    let mut next_index = 1;
    let mut numbers = vec![0];
    let mut grid: Vec<Vec<usize>> = vec![]; // contains indices to the numbers
    let mut symbols: Vec<Symbol> = vec![];

    for (row, line) in lines.enumerate() {
        let mut number = 0;
        grid.push(vec![]);
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                number = number * 10 + c.to_digit(10).unwrap() as i32;
                grid[row].push(next_index);
            } else {
                if number != 0 {
                    numbers.push(number);
                    next_index += 1;
                    number = 0;
                }

                if c != '.' {
                    symbols.push(Symbol {
                        row,
                        col,
                        symbol: c,
                    })
                }

                grid[row].push(0);
            }
        }

        if number != 0 {
            numbers.push(number);
            next_index += 1;
        }
    }

    let mut part2 = 0;
    let mut parts = HashSet::new();
    for s in symbols {
        let mut neighboring_parts = HashSet::new();
        // Iterate ovr the neighbors
        for i in s.row - 1..s.row + 2 {
            for j in s.col - 1..s.col + 2 {
                // We can assume that there are no symbols on the edges
                // if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
                //     continue;
                // }
                parts.insert(grid[i][j]);

                if grid[i][j] != 0 {
                    neighboring_parts.insert(grid[i][j]);
                }
            }
        }

        if neighboring_parts.len() == 2 && s.symbol == '*' {
            let mut gear_ratio = 1;
            for n in neighboring_parts {
                gear_ratio *= numbers[n];
            }
            part2 += gear_ratio;
        }
    }

    let mut part1 = 0;
    for n in parts {
        part1 += numbers[n];
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
