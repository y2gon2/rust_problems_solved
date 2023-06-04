// https://www.acmicpc.net/problem/1302
// https://www.acmicpc.net/source/52344658

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

    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();
    let mut titles: Vec<_> = Vec::new();

    for _ in 0..n{
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let title = buf.trim().parse::<String>().unwrap();
        titles.push(title);
    }


    let mut max_count = 0;
    let counts: Vec<_> = titles
        .iter()
        .map(|title| {
            let count = titles.iter().filter(|&t| t == title).count();
            max_count = count.max(max_count);

            (title, count)
        })
        .collect();

    
    let mut best_seller: Vec<_> = counts
        .iter()
        .filter_map(|&(title, count)| (count == max_count).then(|| title))
        .collect();
}