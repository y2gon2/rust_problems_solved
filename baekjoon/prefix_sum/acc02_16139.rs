//! https://www.acmicpc.net/problem/16139
//! 인간-컴퓨터 상호작용
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut result = String::new();
    
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf)?;
    let mut lines = buf.lines();

    let first = b'a';

    let letters = lines.next().unwrap();
    let mut acc = vec![[0usize; 26]; letters.len() + 1];

    for (idx, l) in letters.char_indices() {
        let alphabet_order = (l as u8 - first) as usize;
        acc[idx][alphabet_order] += 1;

        for j in 0..26 {
            acc[idx + 1][j] = acc[idx][j]; 
        }
    }

    let n = lines.next().unwrap().parse::<usize>()?;

    for _ in 0..n {
        let mut line = lines.next().unwrap().split_ascii_whitespace();

        let (letter, from, to) = (
            line.next().unwrap().parse::<char>()?,
            line.next().unwrap().parse::<usize>()?,
            line.next().unwrap().parse::<usize>()?,
        );

        let find = (letter as u8 - first) as usize;
        
        #[allow(unused_assignments)]
        let mut cnt = 0usize;
        if from > 0{
            cnt = acc[to][find] - acc[from - 1][find];
        } else {
            cnt = acc[to][find];
        }

        result = result + &cnt.to_string() + "\n";
    }

    // println!("{:?}", acc);
    write!(output, "{}", result)?;
    Ok(())
}
