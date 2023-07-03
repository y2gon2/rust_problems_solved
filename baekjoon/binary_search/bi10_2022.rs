//! https://www.acmicpc.net/problem/2022
//! 사다리
//! 
//! 풀이과정
//! c 값을 구할 수 있는 아래 공식은 외부 자료를 참고했음.
//!   h1 = (x^2 - 공통밑변^2).sqrt()
//!   h2 = (y^2 - 공통밑변^2).sqrt() 
//!   c = (h1 * h2) / (h1 + h2)
//! 
//! 위 공식을 사용하여 예측한 밑변을 사용하여 구한 예상 c 와 주어진 c 를 비교 -> 이분탐색
//! 
//! 어려운점
//! 1. 위 공식을 정확하게 code 로 구현하는게 능력이 아직 좀 부족하다.
//! 2. 예측 밑변 길이와 계산된 c 는 서로 반비례 한다. 이를 감안하여 최소 최대값을 조절해야
//! 3. 실수 계산에서 발생하는 오차를 감안, 오차허용범위 이상 자리수를 높여서 정수로 계산하고자 했으나
//!    이럴 경우, 제곱 계산 때문에 허용범위를 넘어감. (u128 은 가능할지도...)
//! 4. 오차 범위를 감안하여, 오차허용범위보다 아래 자리수까지 탐색하여 정답을 찾아야. 

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;

fn binary_search(x:f64, y:f64, c:f64) -> f64 {
    let mut mn = 1f64;
    let mut mx = x.min(y);

    let mut result = 0f64;

    while mn <= mx {
        let mid = (mn + mx) / 2.0;
        
        let h1 = (x.powf(2.0) - mid.powf(2.0)).sqrt();
        let h2 = (y.powf(2.0) - mid.powf(2.0)).sqrt();
        let expected_c = (h1 * h2) / (h1 + h2);

        // println!("mn:{:.4}   mx:{:.4}   mid:{:.4}  c:{:.4}  ex_c:{:.4}", mn, mid, mx, c, expected_c);
        if  (expected_c * 100000.0) as usize == (c * 100000.0) as usize {
            result = mid;
            break;
        } else if expected_c < c {
            mx = mid - 0.00001;
        } else {
            mn = mid + 0.00001;
        }
        result = mid;
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(stdin().lock())?;
    let info: Vec<f64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    let result = binary_search(info[0], info[1], info[2]);
    // let round_reult = result * 1000

    let mut output = stdout().lock();
    writeln!(output, "{:.3}", result)?;
    Ok(())
}