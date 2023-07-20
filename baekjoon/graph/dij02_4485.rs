//! https://www.acmicpc.net/problem/4485
//! 녹색 옷 입은 애가 젤다지?
//! 

use std::io::{stdin, stdout, Read};
use std::error::Error;
use std::str::SplitAsciiWhitespace;






// -------------------------


fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    
    let mut get_usize = || input
        .next()
        .unwrap()
        .parse::<usize>();
    
    let mut get_matrix = |n:usize, input: &mut SplitAsciiWhitespace<'_>| {
        let mut matrix = vec![vec![0u16; n]; n];
        
        for i in 0..n {
            for j in 0..n {
                matrix[i][j] = input.next().unwrap().parse::<u16>().unwrap();
            }
        }
        matrix
    };

    let times = get_usize()?;
    for _ in 0..times {
        let n = get_usize()?;
        let matrix = get_matrix(n, &mut input);
    }


    Ok(())
}