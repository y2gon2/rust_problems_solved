//! https://www.acmicpc.net/problem/9465
//! 스티커

use std::io::stdin;
use std::fmt::Write;
use std::error::Error;
use std::num::ParseIntError;
use std::cmp::max;

fn get_line_info(lines: &mut std::io::Lines<std::io::StdinLock<'_>>) 
    -> Result<Vec<usize>, ParseIntError> {
    let result: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| e)?;

    Ok(result)
}

struct Sticker {
    n: usize,
    cell1: Vec<usize>,
    cell2: Vec<usize>,
    acc1: Vec<usize>,
    acc2: Vec<usize>,
}

impl Sticker {
    fn create_sticker(lines: &mut std::io::Lines<std::io::StdinLock<'_>>) 
        -> Result<Self, Box<dyn Error>> {
            let n = get_line_info(lines)?[0];
            let cell1 = get_line_info(lines)?;
            let cell2 = get_line_info(lines)?;
            let acc1 = cell1.clone();
            let acc2 = cell2.clone();

        Ok(
            Self { n, cell1, cell2, acc1, acc2 }
        )
    }

    fn find_max(&mut self) -> usize {
        if self.n > 1 {
            self.acc1[1] = max(
                self.acc2[0] + self.cell1[1], 
                self.cell1[0]
                );
            self.acc2[1] = max(
                self.acc1[0] + self.cell2[1], 
                self.cell2[0]
                );

            if self.n > 2 {
                for i in 2..self.n {
                    self.acc1[i] = max(
                        max(
                            self.acc1[i - 2] + self.cell2[i - 1],
                            self.acc2[i - 2]
                            ) + self.cell1[i], 
                            self.acc1[i - 1]
                        );
                    self.acc2[i] = max(
                        max(
                            self.acc2[i - 2] + self.cell1[i - 1],
                            self.acc1[i - 1]
                        ) + self.cell2[i],
                        self.acc2[i - 1]
                    );
                }
            }
        } 
        println!("\n{:?}\n{:?}\n---------", self.acc1, self.acc2);
        return max(self.acc1[self.n - 1], self.acc2[self.n - 1])
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = stdin().lines();
    let mut result = String::new();

    let t = get_line_info(&mut lines)?[0];

    for _ in 0..t {
        let mut sticker = Sticker::create_sticker(&mut lines)?;
        writeln!(result, "{}", sticker.find_max())?;
    }

    println!("{result}");
    Ok(())
}