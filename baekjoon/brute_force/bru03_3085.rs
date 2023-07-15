// //! https://www.acmicpc.net/problem/3085
// //! 사탕 게임
// //! 

// use std::io::{stdin, stdout, Read, Write};
// use std::error::Error;

// fn get_input() -> Result<(usize, Vec<Vec<char>>), Box<dyn Error>> {
//     let mut buf = String::new();
//     let _ = stdin().lock().read_to_string(&mut buf);

//     let mut input = buf.split_ascii_whitespace();
//     let mut get_str = || input.next().unwrap();

//     let n = get_str().parse::<usize>()?;

//     let mut board = vec![vec!['A'; n]; n];
//     for i in 0..n {
//         let row = get_str();
//         for (idx, c) in row.char_indices() {
//             // board[i][idx] = c;
//         }
//     }

//     Ok((n, board))
// }

// fn brute_force(n: usize, mut board: Vec<Vec<char>>) -> u8 {
//     let mut result = 0u8;

//     for i in 0..n {
//         result = result.max(line_counter(i, (0, n), &board, false));
//         if result == n as u8 { return result; }    
//     }

//     for i in 0..n {
//         result = result.max(line_counter(i, (0, n), &board, true));
//         if result == n as u8 { return result; }    
//     }
    
//     for i in 0..n - 1 {
//         for j in 0..n - 1 {
//             let temp = board[i][j];
//             board[j][j] = board[j][i + 1];
//             board[j][j + 1] = temp;
             
//             result = result.max(line_counter(i, (0, n), &board, false));
//             if result == n as u8 { return result; }  
//         }
//     }

//     result
// }
// // 
// fn line_counter(line: usize, (from, to):(usize, usize), board: &Vec<Vec<char>>, switch: bool) -> u8 {
//     let mut result = 0u8;
//     let mut color = [0u8; 4];  // Red, Blue, Green, Yellow

//     for j in from..to {
//         let mut y = line;
//         let mut x = j; 
//         if switch {
//             y = j;
//             x = line;
//         }
//         match board[i][j] {
//             'C' => color[0] += 1,
//             'P' => color[1] += 1,
//             'Z' => color[2] += 1,
//             'Y' => color[3] += 1,
//             _ => (),
//         }
//     }
    
//     for c in color {
//         result = result.max(c);
//     }

//     result
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let (n, board) = get_input()?;

//     let mut output = stdout().lock();
//     writeln!(output, "{}", brute_force(n, board))?;

//     Ok(())
// }