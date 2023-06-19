use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::io::{stdin, stdout, BufWriter, prelude::*};
use std::fmt::Write as fWrite;
use std::ops::Range;

const N_MAX: usize = 100_000;
const K_MAX: usize = 10_000;

fn main() {
    solve();
}

fn solve() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u64>);
    // let mut rng = thread_rng();
    // let mut input: Vec<u64> = vec![K_MAX as u64, rng.gen_range(K_MAX..N_MAX) as u64];
    // for _ in 0..K_MAX {
    //     input.push(rng.gen_range(0..2_000_000_000));
    // }
    // let mut input = input.into_iter();

    let (k, n) = (input.next().unwrap() as usize, input.next().unwrap());
    let mut nums = [0u64; K_MAX];
    let mut sum = 0u64;
    for num in nums.iter_mut().take(k) {
        *num = input.next().unwrap();
        sum += *num;
    }

    let mut start = 1u64;
    let mut end = sum / n + 1;
    let mut max_len = u64::MIN;
    'outer: while start < end {
        let middle_len = start + (end - start) / 2;
        let leftover = sum - (middle_len * n);
        let mut leftover_sum = 0u64;

        // can't divide
        for num in nums.iter().take(k) {
            leftover_sum += *num % middle_len;
            if leftover_sum > leftover {
                end = middle_len;
                continue 'outer;
            }
        }
        // can divide
        max_len = max(middle_len, max_len);
        start = middle_len + 1;
    }

    println!("{max_len}");
}
