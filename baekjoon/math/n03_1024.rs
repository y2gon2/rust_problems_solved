//! https://www.acmicpc.net/problem/1024
//! 수열의 합

use std::io::{stdin, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output= String::from("-1");
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut info = buf.split_ascii_whitespace();
    let mut get_n = || info
        .next()
        .unwrap()
        .parse::<i32>();
 
    let (n, length) = (get_n()?, get_n()?);

    for l in length..=100 {
        if (n - (l * (l - 1) / 2)) % l == 0 {
            let a = (n - (l * (l - 1) / 2)) / l;

            if a >= 0 {
                output.clear();
                for i in a..a + l {
                    output += &format!("{} ", i);
                }  
            }
            break;
        }
    }

    println!("{output}");
    Ok(())
}