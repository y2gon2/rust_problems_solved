// https://www.acmicpc.net/problem/1920

use std::io::stdin;

fn main() {
    // input 1st line
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    // input 2nd line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let numN = buffer
        .split_ascii_whitespace()
        .map(|s|)



}