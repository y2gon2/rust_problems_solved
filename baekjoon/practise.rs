// fn read_line(buf: &mut String) {
//     buf.clear();
//     std::io::stdin().read_line(buf).unwrap();
// }

// fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
//     (0..n)
//         .map(|_| {
//             buf.trim().to_string()
//         })
//         .collect()
// }

fn main() {
    let mut buf = String::new();
    // read_line(&mut buf);

    // let n = buf.trim().parse::<i32>().unwrap();
    // let titles = parse_str_vec_lines(&mut buf, n);

    std::io::stdin().read_line()


    let mut max_count = 0;
    let counts: Vec<_> = titles
        .iter()
        .map(|title| {
            let count = titles.iter().filter(|&t| t == title).count();
            max_count = count.max(max_count);

            (title, count)
        })
        .collect();

    for x in 0..n as usize {
        println!("{} : {}", counts[x].0, counts[x].1);
    }
}