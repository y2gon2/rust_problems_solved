// https://www.acmicpc.net/problem/2512
// 예산

use std::io::{stdin, stdout, BufRead, BufWriter, Write, Read};
use std::error::Error;

fn read_n(input: Option<String>) -> usize {
    let mut result: usize = 0;
    if let Some(x) = input {
        result = x.parse::<usize>().unwrap();
    };
    result
}  

fn read_v(input: Option<String>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    if let Some(x) = input {
        result = x.split_ascii_whitespace().map(|mut s| s.parse::<usize>().unwrap()).collect();
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut input = stdin()
        .lock()
        .lines()
        .map(|line| match line {
            Ok(s) => s,
            Err(e) => {
                let msg = e.to_string();
                println!("{}", msg);
                msg
            },
        });

    let n = read_n (input.next());        
    let mut budgets: Vec<usize> = read_v(input.next());
    let total = read_n(input.next());

    budgets.sort();

    let mut mn = 1;
    let mut mx = budgets[n - 1];
    
    while mn <= mx {
        let mid = (mn + mx) / 2;
        let mut sum = 0;
        for i in 0..n {
            if budgets[i] < mid {
                sum += budgets[i];
            } else {
                sum += mid * (n - i);
                break;
            }
        }

        // println!("mn:{}\t mid:{}\tmx:{}", mn, mid, mx);
        if sum <= total {
            mn = mid + 1;
        } else {
            mx = mid - 1;
        }
    }

    writeln!(output, "{}", mx)?;

    Ok(())
}