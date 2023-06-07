use std::io::*;

fn main() -> Result<()> {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut cnt = 0;
    let mut total = 0;

    for i in 1..n + 1 {
        total += i;
        cnt += 1;
        if total == n {
            break;
        } else if total > n {
            cnt -= 1;
            break;
        }
    }

    println!("{}", cnt);

    Ok(())
} 