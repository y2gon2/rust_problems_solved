//! https://www.acmicpc.net/problem/4358
//! 생태학
//! 
//! 

use std::io::{stdin, BufReader, BufRead};
use std::fmt::Write;
use std::error::Error;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let mut buf_reader = BufReader::new(stdin().lock());
    let _ = buf_reader.read_to_string(&mut buf)?;
    let mut input = buf.lines();

    let mut map = BTreeMap::<String, usize>::new();
    let mut total = 0usize;
    while let Some(t) = input.next() {
        total += 1;
        
        map.entry(t.to_string()).and_modify(|cnt | *cnt += 1).or_insert(1);
    }    

    let mut output = String::new();

    for (tree, cnt) in map { 
        let rate: f32 = cnt as f32 / total as f32 * 100.0;
        writeln!(output, "{tree} {rate:.4}")?;
    }

    print!("{output}");
    Ok(())
}