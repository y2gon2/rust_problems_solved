//! https://www.acmicpc.net/problem/1946
//! 신입사원
//! 
//! 풀이과정
//! 1년전 못풀었던 문제
//! 
//! 첫번째 평가 등수에서 자신보다 높은 순위 사람들과만 두번째 평가 등수 비교 진행
//! 만약 두번째 등수도 본인이 낮다면 채용 X
//! -> 첫번째 평가 등수로 정렬한 후 본인의 앞 등수 사람들과만 비교 진행
//! 
//! 1. 해당 조건으로 모든 비교 시 O(N^2) 으로 시간초과 발생
//! 2. 앞의 등수 친구들을 다 비교할 필요없이 앞에 친구들 중 2번째 평가 가장 낮은 등수만 저장
//!    이와 비교 하면 됨.
//!    -> 이를 일반 이중 vec 을 사용하여 진행 
//!       (15520KB / 528ms)
//! 3. 정렬 작업을 제거하기 위해 BTreeSet 을 사용, 오히려 메모리 사용량 증가 및 성능 저하 
//!       (16744KB / 808ms)
//! 4. 첫번째 등수는 정렬에만 사용하므로 이를 memorize 를 위한 index 로 사용 
//!    -> 두번째 평가 등수만 첫번째 등수 위치 vec 에 저장하여 비교 
//!       (13952KB / 424ms)
//! 5. 출력값을 vec에 저장하여 마지막에 String 변환하는 것이 아니라 처음부터 String 으로 저장 변환 과정 제거
//!       (13948KB / 416ms)
//! 6. 입력값을 BufRead 의 memory 사용을 제거 하고 처음부터 일괄 string 으로 가져와서 각각 풀어서 사용
//!   -> 메모리 사용은 크게 증가하였으나 가장 확실한 성능 개선이 발생
//!       (46720KB / 76ms)

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn recruit(n: usize, nums: Vec<usize>) -> usize {
    let mut cnt = 1usize;
    let mut second_min = nums[0];

    for i in 1..n {
        if nums[i] < second_min {
            cnt += 1;
            second_min = nums[i];
        }        
    }
    cnt
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut ans = String::new();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap().parse::<usize>();

    let time = get_num()?;
    for _ in 0..time {
        let n = get_num()?;
        let mut nums = vec![0usize; n];
        
        for _ in 0..n {
            let first = get_num()?;
            let second = get_num()?;
            nums[first - 1] = second;
        }

        ans += &(recruit(n, nums).to_string());
        ans += "\n";
    }

    writeln!(output, "{}", ans)?;
    Ok(())
}