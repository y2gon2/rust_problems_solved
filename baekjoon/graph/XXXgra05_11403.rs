// https://www.acmicpc.net/problem/11403
// 경로 찾기

use std::io::{stdin, stdout, BufWriter, Read};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut result = String::new();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut num = || -> Result<usize, std::num::ParseIntError> {
        input.next().unwrap().parse::<usize>()
    };

    let n = num()?;
    let mut nodes = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            let n = num()?;
            if n == 1 {
                nodes[i][j] = true;
                nodes[j][i] = true;
            }
        }
    }

    let mut visited = vec![vec![false; n]; n];
    
    Ok(())
}
 