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
    let mut num_n: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    num_n.sort();

    // input 3rd line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let m = buffer.trim().parse::<usize>().unwrap();

    // input 4th line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let num_m: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    

    for i in 0..m {
        let mut first = 0;
        let mut last = n - 1;
    
        let mut is_exit = false;

        while first <= last {
            let mid = (first + last) / 2;

            if i == num_m[mid] {
                is_exit = true;
                println!("1");
                break;
            } else if i > num_m[mid] {
                first = mid + 1;
            } else if mid == 0  {
                if i == num_m[mid] {
                    is_exit = true;
                    println!("1");
                    break;
                } else {
                    break;
                }
            } else {
                last = mid - 1;
            }
        } 
        
        if !is_exit {
            println!("0");
        }
    }
}