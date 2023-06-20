// https://www.acmicpc.net/problem/1166
// 선물 

use std::io::{BufWriter, stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf)?;
    let mut size: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>())
        .collect::<Result<_, _>>()?;

    let n = size.remove(0);
    size.sort();

    let mut mn = 0.0;
    let mut mx = size[0] as f64;

    for _ in 0..61 {
        let mid = (mn + mx) / 2.0;
        let boxes = (size[0] as f64 / mid) as usize * (size[1] as f64 / mid) as usize * (size[2] as f64 / mid) as usize; 
        
        if boxes >= n {
            mn = mid;
        } else {
            mx = mid;
        }

        // println!("mn:{} mid:{} mx:{}", mn, mid, mx);
    }

    writeln!(output, "{:.9}", mx)?;

    Ok(())
}