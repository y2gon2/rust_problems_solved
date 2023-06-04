// https://www.acmicpc.net/problem/10815

use std::io::*;

fn main() -> Result<()>{
    let mut buffer = String::new();

    // input first line
    stdin().read_line(&mut buffer).unwrap();
    let card_num = buffer.trim().parse::<usize>().unwrap();

    // input 2nd line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let mut cards: Vec<i32> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    cards.sort();

    // input 3rd line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let num_of_searching_num = buffer.trim().parse::<usize>().unwrap();

    // input 4th line
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let searching_nums: Vec<i32> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();


    // for x in 0..card_num {
    //     print!("{} ", cards[x]);
    // }
    // println!("");

    // 아.. 왜 또 이분 탐색이야.. 
    let mut result: Vec<usize> = Vec::new();

    for i in 0..num_of_searching_num {
        let mut left: usize = 0;
        let mut right: usize = card_num - 1;
        let mut status: bool = false;

        while left <= right {
            let mid = (left + right) / 2;
            // println!("searching num:{}  left:{}  mid:{}({})  right:{}", searching_nums[i], left, mid, cards[mid], right);

            if searching_nums[i] == cards[mid] {
                result.push(1);
                // println!("{}", searching_nums[i]);
                status = true;
                break;
            } else if searching_nums[i] < cards[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        if !status {
            result.push(0);
        }
    }

    for j in 0..num_of_searching_num {
        
        print!("{} ", result[j]);
    }

    Ok(())
}