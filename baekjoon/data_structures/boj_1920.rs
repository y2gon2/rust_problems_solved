// https://www.acmicpc.net/problem/1920

use std::io::stdin;

fn main() {
    // input 1st line
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let _n = buffer.trim().parse::<usize>().unwrap();

    // input 2nd line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let num_n: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // input 3rd line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let _m = buffer.trim().parse::<usize>().unwrap();

    // input 4th line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let num_m: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut status = false;

    for i in num_m {
        for j in num_n.clone() {
            if i == j {
                println!("1");
                status = true;
                break;
            }
        }
        if !status {
            println!("0");
        }
    }
}