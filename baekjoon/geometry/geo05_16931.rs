// // https://www.acmicpc.net/problem/16931
// // 겉넓이 구하기

// use std::io::{stdin, BufRead, Result};
// use std::cmp::max;

// fn main() -> Result<()> {
//     let mut buf = String::new();
//     stdin().lock().read_line(&mut buf).unwrap();
//     let nums: Vec<usize> = buf
//         .split_ascii_whitespace()
//         .map(|s| s.parse::<usize>().unwrap())
//         .collect();

//     let n = nums[0];
//     let m = nums[1];

//     let mut result = (n * m * 2) + (n + m) * 2;

//     let mut blocks: Vec<Vec<usize>> = vec![vec![0; m + 2]; n + 2];
//     let mut height: usize = 0;

//     for i in 1..n + 1 {
//         buf.clear();
//         stdin().lock().read_line(&mut buf).unwrap();
//         let row: Vec<usize> = buf
//             .split_ascii_whitespace()
//             .map(|s| s.parse::<usize>().unwrap())
//             .collect();

//         for (j, r) in row.iter().enumerate() {
//             blocks[i][j + 1] = *r - 1;  
//             height = max(height, *r - 1);
//         }
//     }

//     for _ in 0..height {
//         for y in 1..n + 1 {
//             for x in 1..m + 1{
//                 if blocks[y][x] > 0 {
//                     if blocks[y - 1][x] == 0 { result += 1; }
//                     if blocks[y + 1][x] == 0 { result += 1; }
//                     if blocks[y][x - 1] == 0 { result += 1; }
//                     if blocks[y][x + 1] == 0 { result += 1; }
//                 }
//             }
//         }    

//         for y1 in 1..n + 1 {
//             for x1 in 1..m + 1{
//                 if blocks[y1][x1] > 0  {
//                     blocks[y1][x1] -= 1;
//                 }
//             }
//         }
//     }

//     println!("{}", result);

//     Ok(())
// }
use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(io::stdout().lock());
    let mut line = || input.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let first_line = line();

    let mut area = first_line[0] * first_line[1] * 2;
    let v = (0..first_line[0]).map(|_| {
        let vec = line();
        let len = vec.len();
        area += vec[0] + vec[len-1];
        for i in 0..len-1 {
            area += (vec[i] - vec[i+1]).abs()
        }
        vec
    })
        .collect::<Vec<Vec<i32>>>();

    area += transpose_matrix(&v);

    writeln!(output, "{}", area)?;

    Ok(())
}

fn transpose_matrix(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut area = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut vec = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            vec[j][i] = matrix[i][j];
        }
    }

    for v in &vec {
        let len = v.len();
        area += v[0] + v[len-1];
        for i in 0..len-1 {
            area += (v[i] - v[i+1]).abs()
        }
    }

    area
}