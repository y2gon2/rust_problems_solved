// https://www.acmicpc.net/problem/1213
// 팰린드롬 만들기
// soruce: https://www.acmicpc.net/source/51758204

use std::collections::BTreeMap;
use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let mut map = BTreeMap::<char, usize>::new();

    buffer.trim().chars().for_each(|c| {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });

    let odds: Vec<_> = map
        .iter()
        .filter(|(_, &v)| v % 2 == 1)
        .map(|(k, _)| k)
        .collect();

    if odds.len() > 1 {
        println!("Sorry");
        return ;
    }

    let mut palindrome = match odds.get(0) {
        None => String::new(),
        Some(k) => k.to_string(),
    };

    for (&k, &v) in map.iter().rev() {
        for _ in 0..v / 2 {
            palindrome.push(k);
            palindrome.insert(0, k);
        }
    }

    println!("{palindrome}");
}