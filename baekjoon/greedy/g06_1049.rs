// // https://www.acmicpc.net/problem/1049
// // 기타줄

// use std::io::{stdin, Result};
// use std::cmp::min;

// const SET_WIRES: usize = 6;

// fn main() -> Result<()> {
//     let mut buffer: String = String::new();
//     stdin().read_line(&mut buffer).unwrap();
//     let v1: Vec<usize> = buffer
//         .split_ascii_whitespace()
//         .map(|s| s.parse::<usize>().unwrap())
//         .collect();

//     let n: &usize = v1.get(0).unwrap();
//     let m: &usize = v1.get(1).unwrap(); 

//     let mut set_price: usize = 1_001;
//     let mut wire_price: usize = 1_001;
//     for _ in 0..*m {
//         buffer.clear();
//         stdin().read_line(&mut buffer).unwrap();
//         let v2: Vec<usize> = buffer
//             .split_ascii_whitespace()
//             .map(|s| s.parse::<usize>().unwrap())
//             .collect();

//         set_price = min(set_price, *v2.get(0).unwrap());
//         wire_price = min(wire_price, *v2.get(1).unwrap());
//     }

//     set_price = min(set_price, wire_price * 6);

//     let lowest_price = {
//         (n / SET_WIRES) * set_price 
//         +
//         min((n % SET_WIRES) * wire_price, set_price)
//     };

//     println!("{}", lowest_price);

//     Ok(())
// }

use std::io::{stdin, BufRead};

fn main() {
    let mut input = String::new();

    stdin().lock().read_line(&mut input).ok();

}