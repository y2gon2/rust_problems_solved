//! https://www.acmicpc.net/problem/14465
//! 소가 길을 건너간 이유 5

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = u32::MAX;
    let mut output = stdout().lock();
    
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap().parse::<usize>();

    let (n, k, _b) = (get_num()?, get_num()?, get_num()?);

    let brokens = input.flat_map(|s| s.parse::<u32>()).collect::<HashSet<_>>();

    let mut crosses = vec![0u32; n + 1];
    for i in 1..=n {
        if brokens.contains(&(i as u32)) {
            crosses[i] += 1;
        }
        crosses[i] += crosses[i - 1];

        if i >= k {
            let to_be_fixed = crosses[i] - crosses[i - k];

            if to_be_fixed == 0 {
                result = 0;
                break;
            } else {
                result = result.min(to_be_fixed);
            }
        }
    }

    writeln!(output, "{}", result)?;
    Ok(())
}