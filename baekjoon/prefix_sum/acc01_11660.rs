//! https://www.acmicpc.net/problem/11660
//! 구간 합 구하기 5
//! 
//! 풀이과정
//! 누적합에 대한 개념없이 바로 폴었을 때 시간 초과 발생
//! 
//! matrix 를 누적형태로 구성하는 것은 쉽게 생각했으나,
//! 원활한 풀이를 진행하기 까지 상세 구현에서 많은 시간이 소요됨.
//! 1. 누적헙으로 구성된 matix 에서 원하는 부분에 대한 합을 구하기 위해서 
//!    적절한 부위의 덧셈과 뺄셈 조합을 구성해야 함. 
//! 2. 좌표와 결과값에 대한 뺄셈이 존재하게 되면서 좌표값 또는 value 를 구하는 과정에 음수가 발생할 수 있어짐.
//!    - value 의 경우 뺄셈을 뒤에 배치하여 음수 발생 가능성 제거
//!    - 좌표값의 경우, matrix 생성시 행, 열을 각각 실제 1번 부터 시작하여 음수 좌표값 발생을 제거
//! -> 18948KB . 88ms
//! 
//! 성능 개선
//! 기존 input 을 line 별로 받아 바로 vec 으로 변환하여 이를 사용했으나,
//! 전체 입력을 한번에 받고 해당 string 에 대해 iterator 만 생성하여 data 가 필요한 경우에 
//! 그때 그때 iterator 를 이동시켜서 data 를 받아서 활용 (vec 생성 과정 제거)
//! -> 30760KB / 48ms

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf)?;
    let mut lines = buf.lines();

    let mut output = stdout().lock();
    let mut result = String::new();

    let mut info = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (n, m) = (info.next().unwrap(), info.next().unwrap());

    let mut matrix = vec![vec![0_usize; n + 1]; n + 1];
    for i in 1..=n {
        let mut temp = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

        for j in 1..=n {
            matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1] - matrix[i - 1][j - 1] + temp.next().unwrap();
        }
    }

    // for i in 0..n {
    //     println!("{:?}", matrix[i]);    
    // }

    for _ in 0..m {
        let mut pnt = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

        let (y1, x1, y2, x2) = (
            pnt.next().unwrap(), pnt.next().unwrap(), pnt.next().unwrap(), pnt.next().unwrap()
        );

        // println!("matrix1[{}][{}]: {}, matrix2[{}][{}]: {}", x1, y1, matrix[y1][x1], x2, y2, matrix[y2][x2]);
        
        result = result + &(matrix[y2][x2] + matrix[y1 - 1][x1 - 1] - matrix[y2][x1 - 1] - matrix[y1 - 1][x2]).to_string() + "\n";
    }

    writeln!(output, "{}", result)?;

    Ok(())
}
