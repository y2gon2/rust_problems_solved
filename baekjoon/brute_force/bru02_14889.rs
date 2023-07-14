//! https://www.acmicpc.net/problem/14889
//! 스타트와 링크
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::min;

fn get_input() -> Result<(usize, Vec<Vec<u8>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap();

    let n = get_num().parse::<usize>()?;

    let mut matrix = vec![vec![0u8; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            if i == j { 
                let _ = get_num().parse::<u8>()?;
            } else if j > i {
                matrix[i][j] += get_num().parse::<u8>()?;
            } else {
                matrix[j][i] += get_num().parse::<u8>()?;
            }
        }
    } 

    // for i in 1..=n {
    //     println!("{:?}", matrix[i]);
    // }

    Ok((n, matrix))
}

fn back_tracking(
    cnt: usize,
    this: usize,
    n: usize, 
    matrix: &Vec<Vec<u8>>, 
    players: &mut Vec<bool>, 
    result: &mut usize
) {
    if cnt == n / 2 {
        let mut team1 = 0usize;
        let mut team2 = 0usize;

        for i in 1..=n {
            for j in (i + 1)..=n {
                if players[i] && players[j] { team1 += matrix[i][j] as usize; }
                if !players[i] && !players[j] { team2 += matrix[i][j] as usize; }
            }
        } 
        let diff = (team1 as i32 - team2 as i32).abs() as usize;

        // println!("team1:{},\t  team2:{},\t  diff:{},\t  players:{:?}", team1, team2, diff, players);
        *result = min(*result, diff);
    }

    for i in this..n {
        players[i] = true;
        back_tracking(cnt + 1, i + 1, n, matrix, players, result);
        players[i] = false;
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let (n, matrix) = get_input()?;
    let mut result = usize::MAX;
    let mut players = vec![false; n + 1];

    back_tracking(0, 1, n, &matrix, &mut players, &mut result);

    let mut output = stdout().lock();
    writeln!(output, "{}", result)?;

    Ok(())
}