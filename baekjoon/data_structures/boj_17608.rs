// https://www.acmicpc.net/problem/17608

use std::io::*;

fn main() -> Result<()>{
    let mut stdin = BufReader::new(stdin().lock());
    //let mut stdout = BufWriter::new(stdout().lock());

    let mut s = String::new();
    stdin.read_to_string(&mut s).unwrap();

    let input = s
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    
    let bars: Vec<usize> = input.skip(1).collect();
    let bars_iter = bars.iter().rev();
    let mut tallest: usize = 0;
    let mut result: usize = 0;

    for bar in bars_iter {
        if bar > &tallest {
            result += 1;
            tallest = bar.clone();
        }
    }

    println!("{}", result);

    Ok(())
}


// reference code  ID: falsetru https://www.acmicpc.net/source/45776489
// use std::io::{self, *};

// fn main() -> Result<()> {
//     let stdin = io::stdin();
//     let stdout = io::stdout();
//     let mut stdin = io::BufReader::new(stdin.lock());
//     let mut stdout = io::BufWriter::new(stdout.lock());

//     let mut s = String::new();
//     stdin.read_to_string(&mut s)?;

//     let it = s
//         .split_ascii_whitespace()
//         .map(|x| x.parse::<usize>().unwrap());

//     let x: Vec<_> = it.skip(1).collect();
//     let mut it = x.iter().rev();
//     let mut max = it.next().unwrap();
//     let mut ans = 1;
//     for x in it {
//         if x > max {
//             ans += 1;
//             max = x;
//         }
//     }

//     writeln!(stdout, "{ans}")?;

//     Ok(())
// } 


// 1st passing code 

// fn main() {
//     let mut buffer = String::new();
//     stdin().read_line(&mut buffer).unwrap();
//     let n = buffer.trim().parse::<usize>().unwrap();
//     let mut bars: Vec<usize> = Vec::new();

//     for _ in 0..n {
//         buffer.clear();
//         stdin().read_line(&mut buffer).unwrap();
//         let bar = buffer.trim().parse::<usize>().unwrap();
//         bars.push(bar);
//     }

//     let mut base_line = bars[n-1];

//     let mut result = 1;
//     for i in 0..n - 1 {
//         if bars[n - 2 - i] > base_line {
//             result += 1;
//             base_line = bars[n - 2 - i];
//         } 
//     }

//     println!("{}", result);
// }
 
