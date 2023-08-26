//! https://www.acmicpc.net/problem/9251
//! LCS(Longest Common Subsequence, 최장 공통 부분 수열)
//! (두 수열이 주어졌을 때, 모두의 부분 수열이 되는 수열 중 가장 긴 것을 찾는 문제)

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::max;

fn find(a1: &Vec<char>, a2: &Vec<char>, idx: usize, i: usize, j: usize, acc: &mut Vec<usize>) {
    for sub_i in i..a1.len() {
        for sub_j in j..a2.len() {
            if a1[sub_i] == a2[sub_j] {
                // println!("idx: {}, a1[{}] :{}", idx, sub_i, a1[sub_i]);
                acc[idx] += 1;

                if sub_i + 1 < a1.len() && sub_j + 1 < a2.len() {
                    find(a1, a2, idx, sub_i + 1, sub_j + 1, acc);
                } 
                return
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut result = 0usize;

    let mut buf = String::new();
    
    let _ = stdin().read_line(&mut buf);
    let a1: Vec<char> = buf.trim().chars().collect();

    buf.clear();
    let _ = stdin().read_line(&mut buf);
    let a2: Vec<char> = buf.trim().chars().collect();
    
    let mut acc1 = vec![0usize; a1.len()];
    for idx in 0..a1.len() {
        // let mut temp_acc = [0usize; 1];
        find(&a1, &a2, idx, idx, 0, &mut acc1);
        // result = max(result, temp_acc[0]);
    }

    println!("acc1: {:?}", acc1);
    for v in acc1.iter() {
        result = max(result, *v);
    }

    let mut acc2 = vec![0usize; a2.len()];
    for idx in 0..a2.len() {
        // let mut temp_acc = [0usize; 1];
        find(&a2, &a1, idx, idx, 0, &mut acc2);
        // result = max(result, temp_acc[0]);
    }

       println!("acc2: {:?}", acc2);
    for v in acc2.iter() {
        result = max(result, *v);
    }

    writeln!(output, "{}", result)?;
    Ok(())
}