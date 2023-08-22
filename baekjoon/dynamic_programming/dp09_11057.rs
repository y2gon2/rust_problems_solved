//! https://www.acmicpc.net/problem/11057
//! 오르막 수
//! 
//! 각 자리수 : 0	1	2	3	4	5	6	7	8	9
//! 1의 자리수: 1	1	1	1	1	1	1	1	1	1
//! 2의 자리수: 1	2	3	4	5	6	7	8	9	10
//! 3의 자리수: 1	3	6	10	15	21	28	36	45	55
//! 4의 자리수: 1	4	10	20	35	56	84	120	165	220
//! 
//! 위와 같이 문제에서 요청하는 값은 각 자리수의 0~9 까지에서 
//! 각 자리수의 숫자보다 작거나 같은 숫자가 지닌 값의 합과 같다.
//! 예를 들면, 1의 자리수 오르막은 0~9 의 첫째자리수가 가진 값의 합 : 10
//!            2의 자리수 오르막은 0~9 의 첫째자리수가 가진 값의 합 : 55 
//! ...

use std::io::{stdin, stdout, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    let n = buf.trim().parse::<usize>()?;
    let mut acc = vec![[1u16; 10]; n + 1];
    
    for i in 1..=n {
        for j in 0..10 {
            let mut temp = 0u16;
            for k in 0..=j {
                temp += acc[i - 1][k];
            }
            acc[i][j] = temp % 10_007;
        }
    }

    writeln!(output, "{}", acc[n][9])?;
    Ok(())
}