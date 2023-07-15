//! https://www.acmicpc.net/problem/1041
//! 주사위
//! 
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn get_input() -> Result<(usize, Vec<usize>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap().parse::<usize>();

    let n = get_num()?;
    let mut dice = vec![0usize; 6];

    for i in 0..6 {
        dice[i] = get_num()?;
    }

    Ok((n, dice.sort()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (n, dice) = get_input()?;

    let blocks = n * n * n;
    let face3 = dice[0] + dice[1] + dice[2];
    let face2 = dice[0] + dice[1];
    let mut result =  


    Ok(())
}