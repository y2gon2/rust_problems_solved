//! https://www.acmicpc.net/problem/11656
//! 접미사 배열

use std::io::{stdin, Read};
use std::fmt::Write;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let word: Vec<char> = buf
        .trim()
        .chars()
        .collect();

    let mut list = vec![String::new(); word.len()];
    let mut temp = String::new();
    for i in (0..word.len()).rev() {
        temp.insert(0, word[i]);
        list[i]  = temp.clone();
    }

    list.sort();
    println!("{:?}",  list);

    for w in list.iter() {
        writeln!(output, "{}", w)?;
    }

    print!("{output}");
    Ok(())
}