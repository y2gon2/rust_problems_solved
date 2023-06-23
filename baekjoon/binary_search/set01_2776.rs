// https://www.acmicpc.net/problem/2776
// 암기왕

use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut output = BufWriter::new(stdout().lock());
    let mut result = String::new();

    let mut input: Vec<i32> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()?;

    let n = input.remove(0);
    for _ in 0..n {
        let note1_n = input.remove(0);
        let mut note1: Vec<i32> = input.drain(0..note1_n as usize).collect();
        let note2_n = input.remove(0);
        let note2: Vec<i32> = input.drain(0..note2_n as usize).collect();

        // println!("{:?} {:?} {:?} {:?}", note1_n, note1, note2_n, note2);
        note1.sort();

        for n in note2.iter() {
            let mut status = false;
            let mut left: usize = 0;
            let mut right: usize = note1_n as usize - 1;
            while left <= right {
                let mid = (left + right) / 2;
                // println!("n:{}\tleft:{}\t mid:{}\t right:{}", n, left, mid, right);
    
                if &note1[mid] == n {
                    status = true;
                    break;
                } else if &note1[mid] < n {
                    left = mid + 1;
                } else {
                    if mid > 0 {
                        right = mid - 1;
                    } else {
                        break;
                    }
                }
            }
            if status {
                result += "1\n";
            } else {
                result += "0\n";
            }
        }
    }
    writeln!(output, "{}", result)?;
    Ok(())
}