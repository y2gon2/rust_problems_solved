//! https://www.acmicpc.net/problem/1041
//! 주사위

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::{min, max};

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

    Ok((n, dice))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0usize;
    let mut output = stdout().lock();
    let (n, dice) = get_input()?;

    let mut min3 = usize::MAX;
    let mut min2 = usize::MAX;
    let mut min1 = usize::MAX;
    for i in 0..6 {
        min1 = min(min1, dice[i]);
        for j in (i + 1)..6 {
            if (i == 0 && j == 5) || (i == 1 && j == 4) || (i == 2 && j == 3) { continue; }
            min2 = min(min2, dice[i] + dice[j]);

            for k in (j + 1)..6 {
                if (i == 0 && k == 5) || (i == 1 && k == 4) || (j == 1 && k == 4) || (j == 2 && k == 3) { continue; }
                min3 = min(min3, dice[i] + dice[j] + dice[k]);
            }
        }
    }

    match n {
        1 => {
            let mut mx = 0usize;
            for i in 0..6{
                result += dice[i];
                mx = max(mx, dice[i]);
            }
            result -= mx;
        },
        2 => result = min3 * 4 + min2 * 4,
        _ => result = min3 * 4 
            + (n - 2) * min2 * 4 
            + (n - 1) * min2 * 4 
            + (n - 1) * (n - 2) * min1 * 4
            + (n - 2) * (n - 2) * min1, 
    }
    writeln!(output, "{}", result)?;
    Ok(())
}