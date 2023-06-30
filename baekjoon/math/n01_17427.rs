//! https://www.acmicpc.net/problem/17427
//! 약수의 합 2
//! 10 일때 약수의 합을 구해보면 다음과 같은 규칙이 확인된다. 
//! f(1) = sum(1) 
//! f(2) = sum(1, 2)
//! f(3) = sum(1, 3)
//! f(4) = sum(1, 2, 4)
//! f(5) = sum(1, 5)
//! f(6) = sum(1, 2, 3, 6)
//! f(7) = sum(1, 7)
//! f(8) = sum(1, 2, 4, 8)
//! f(9) = sum(1, 3, 9)
//! f(10) = sum(1, 2, 5, 10)
//! 
//! 위 결과를 더해지는 숫자별로 정리해 보면 다음과 같다.
//! 1: 10 개 == 10 / 1
//! 2:  5 개 == 10 / 2
//! 3:  3 개 == 10 / 3
//! 4:  2 개 == 10 / 4
//! 5:  2 개 == 10 / 5
//! ...
//! 9:  1 개 == 10 / 9
//! 10: 1 개 == 10 / 10 


use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let input = read_to_string(stdin().lock())?;
    let num = input.trim().parse::<usize>()?;

    let mut sum = 0usize;

    for i in 1..=num {
        sum += (num / i) * i;
    }

    writeln!(output, "{}", sum)?;

    Ok(())
}