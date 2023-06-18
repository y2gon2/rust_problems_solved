// https://www.acmicpc.net/problem/10815
// 숫자 카드 

use std::io::*;

fn main() -> Result<()>{
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(stdout().lock());

    let mut get_nums = || input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let n = get_nums()[0] as usize;
    let mut cards = get_nums();
    cards.sort();

    let _ = get_nums();
    let my_nums = get_nums();

    let mut result = String::new();

    for my in my_nums {
        let mut status = false;
        let mut mn = 0;
        let mut mx = n - 1;

        while mn <= mx {
            let mid = (mn + mx) / 2;

            // println!("my:{}\t mn:{}\t mid:{}\t mx:{}", my, mn, mid, mx);

            if my == cards[mid] {
                status = true;
                break;
            } else if my > cards[mid] {
               mn = mid + 1;
            } else {
                if mid > 0 { mx = mid - 1; }
                else { break; }
            }
        }

        if status {
            result.push_str("1 ");
        } else {
            result.push_str("0 ");
        }
    }

    writeln!(output, "{}", result)?;

    Ok(())
}