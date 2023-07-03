//! https://www.acmicpc.net/problem/2295
//! 세 수의 합
//! 
//! 풀이 과정
//! 일반적 brute force 방법의 경우 시간 복잡도가 O(N^3 * logN) -> 시간 초과 예상
//! 
//! 1. 이분탐색으로 초반에 방법이 떠오르지 않아 two pointer 로 접근 시간 복잡도 : O(n^3) 
//!    target 과 첫번째 숫자 선정에 각각 O(N) * O(N) 
//!    나머지 두수의 조합을 two pointer 로 탐색 * O(N)
//!    -> 13160KB / 528ms
//! 
//! 2. 다른 코드를 약간 참조하여 이분탐색... 을 시도하려고 하였으나, 
//!    BTreSet 으로 두수의 합의 경우의 수를 담는다면, 내부적으로 이분 분할 자료 구조이므로 
//!    해당 코드 구현 없이 전체 시간 복잡도 O(N^2 * logN) 으로 구현가능해짐.
//!    -> 21229KB / 88ms
//! 
//! 3. 그런데 사실 두수의 합의 조합에서 정렬을 할 필요가 없으므로, HashMap 을 사용하면,
//!    최종 시간복잡도 O(N^2) 으로 처리 가능해짐
//!    -> 27032KB / 44ms

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let input = read_to_string(stdin().lock())?;
    let mut nums: Vec<usize> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    nums.sort_by(|a, b| b.cmp(a));
    
    let n = nums.len();
    let mut b_set = HashSet::<usize>::new();

    for i in 0..n {
        for j in i..n {
            b_set.insert(nums[i] + nums[j]);
        }
    }

    for i in 0..n {
        for j in i..n {
            let rest = nums[i] - nums[j];

            if b_set.contains(&rest) {
                writeln!(output, "{}", nums[i])?;
                return Ok(());
            }
        }
    }
    Err("no matching numbers".into())
}
