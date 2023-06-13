// https://www.acmicpc.net/problem/2477
// 참외밭

use std::io::{BufWriter, stdin, stdout, Result, Read, Write};

fn main() -> Result<()> {
    let mut output = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let n = lines.next().unwrap().trim().parse::<usize>().unwrap();
    let mut values: Vec<Vec<usize>> = Vec::new();
 
    (0..6).for_each(|_| {
        let v: Vec<usize> = lines
            .next()
            .unwrap() 
            .split_ascii_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        
        values.push(v);
    });

    let mut big_sq: usize = 0;
    let mut small_sq: usize = 0;
    for i in 0..6 {
        let turn = (values[i][0], values[(i + 1) % 6][0]);
        if turn == (1, 3) || turn == (3, 2) || turn == (2, 4) || turn == (4, 1)  {
            small_sq = values[i][1] * values[(i + 1) % 6][1];
            big_sq = values[(i + 3) % 6][1] * values[(i + 4) % 6][1];

            break;
        }
    };

    writeln!(output, "{}", (big_sq - small_sq) * n).unwrap();

    Ok(())
}
