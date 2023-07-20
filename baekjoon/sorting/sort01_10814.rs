//! https://www.acmicpc.net/problem/10814
//! 나이순 정렬
//! 

use std::io::{stdin, stdout, Read, Write};
use std::collections::BTreeMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_info = || input.next().unwrap();

    let n = get_info().parse::<usize>()?;
    let mut map = BTreeMap::<u8, Vec<&str>>::new();

    for _ in 0..n {
        let key = get_info().parse::<u8>()?;
        let value = get_info();
        map.entry(key).or_insert_with(Vec::new).push(value);
    }

    let mut result = String::new();
    for (k, v) in map.iter() {
        for name in v {
            let temp = &format!("{} {}\n", k, name);
            result += temp;
        }
    }

    let mut output = stdout();
    writeln!(output, "{}", result)?;
    Ok(())
}