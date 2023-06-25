// https://www.acmicpc.net/problem/3649
// 로봇 프로젝트

use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);    

    loop {
        if buf.is_empty() {
            break;
        } else {
            let mut input: Vec<usize> = buf
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>())
                .collect::<Result<_, _>>()?;

            let mut result = String::from("danger");
            let size = input.remove(0) * 10_000_000;
            let n = input.remove(0);

            if n == 0 || n == 1 {
                println!("{}", result);
                continue;
            }
            // println!("{:?}", input);
            
            let mut blocks: Vec<usize> = input.drain(0..n).collect();
            blocks.sort();

            let mut left: usize = 0;
            let mut right: usize = n - 1;
    
            while left < right {
                let sum = blocks[left] + blocks[right];
                
                if sum == size {
                    result = "yes ".to_string() + &blocks[left].to_string() + " " + &blocks[right].to_string();
                    break;
                } else if sum > size {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
            println!("{}", result);

        }
        buf.clear();
        let _ = stdin().lock().read_to_string(&mut buf);
    }


    Ok(())
}