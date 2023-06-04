use std::io::*;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut num = buffer.trim().parse::<usize>().unwrap();

    let mut result: Vec<usize> = (1..=num)
        .filter(|x| x % 2 == 1)
        .collect();
    let mut card_queue: VecDeque<usize> = (1..=num)
        .filter(|x| x % 2 == 0)
        .collect();

    let mut cnt = 1;
    if num % 2 == 1 { cnt += 1;}

    while !card_queue.is_empty() {
        let temp = card_queue.pop_front().unwrap();
        if cnt % 2 == 1 {
            result.push(temp);
        } else {
            card_queue.push_back(temp);
        }

        cnt += 1;
    }   

    for x in 0..num {
        print!("{} ", result[x]);
    }

    Ok(())
}