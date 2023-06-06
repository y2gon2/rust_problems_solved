// https://www.acmicpc.net/problem/2304

use std::io::*;


fn main () -> Result<()> {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut columns: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let col: Vec<usize> = buffer
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        columns.push(col);
    }

    columns.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut heightest = 0;

    for i in 0..n {
        if columns[i][1] > heightest {
            heightest = columns[i][1];
        }
    }

    let mut area: usize = 0;
    let mut col_height: usize = 0;
    let mut col_start: usize = 0;

    for j in 0..n {
        if columns[j][1] > col_height {
            area += col_height * (columns[j][0] - col_start); 
            col_height = columns[j][1];
            col_start = columns[j][0];
        }

        if j == n - 1 {
            area += col_height * (columns[j][0] + 1 - col_start);
        }
    }

    let heightest: usize = col_height;
    col_height = 0;
    let mut col_end: usize = columns[n - 1][0]; 

    for k in 1..n + 1 {
        if columns[n - k][1] > col_height {
            area -= (heightest - col_height) * (col_end - columns[n - k][0]);
            col_height = columns[n - k][1];
            col_end = columns[n - k][0];
        }
    }

    println!("{}", area);

    Ok(())
}