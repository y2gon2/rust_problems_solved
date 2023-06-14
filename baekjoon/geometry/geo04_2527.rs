// https://www.acmicpc.net/problem/2527
// 직사각형

use std::io::{stdin, Result, BufWriter, Read, stdout, Write};

fn main() -> Result<()> {
    let mut output = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    let mut result: Vec<char> = Vec::new();

    stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    for _ in 0..4 {

        let line: Vec<i32> = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        let a1: (i32, i32) = (line[0], line[1]);
        let a2: (i32, i32) = (line[2], line[3]);
        let b1: (i32, i32) = (line[4], line[5]);
        let b2: (i32, i32) = (line[6], line[7]);

    
        let mut temp = 'a';

        if a1.0 > b2.0 || a1.1 > b2.1 || a2.0 < b1.0 || a2.1 < b1.1 { 
            temp = 'd'; 
        } else if  a1.0 == b2.0 {
            if a1.1 == b2.1 || a2.1 == b1.1 { 
                temp = 'c'; 
            } else {
                temp = 'b';
            }
        } else if a2.0 == b1.0 {
            if a1.1 == b2.1 || a2.1 == b1.1 { 
                temp = 'c';
            } else {
                temp = 'b';
            }
        } else if a1.1 == b2.1 || a2.1 == b1.1 {
            temp = 'b';
        }        
        result.push(temp);
    }

    for j in 0..4 {
        writeln!(output, "{}", result[j]).unwrap();    
    }
    Ok(())
}