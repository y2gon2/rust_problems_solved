// https://www.acmicpc.net/problem/1302

use std::io::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();  

    let mut books: HashMap<String, usize> = HashMap::new();

    for _ in 0..n {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let book = buffer.trim().parse::<String>().unwrap();
        
        match books.get_mut(&book) {
            None => {books.insert(book, 1); ()},
            Some(n) => { *n += 1; ()},
        }
    } 

    let mut books_vec: Vec<_> = books.iter().collect();
    books_vec.sort_by(|a, b| a.0.cmp(b.0));
    books_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{}", books_vec[0].0);

    Ok(())
}