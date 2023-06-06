// https://www.acmicpc.net/problem/4358

use std::io::*;

fn main() {
    let mut buf_reader = BufReader::new(stdin().lock());
    // let mut buf_writer = BufWriter::new(stdout().lock());

    let mut input_string = String::new();

    buf_reader.read_to_string(&mut input_string).unwrap();

    let mut trees: Vec<String> = input_string
        .split('\n')
        .map(|x| x.parse::<String>().unwrap())
        .collect();

    trees.sort();

    let total = trees.len();
    let mut this_tree = String::from(trees[0].clone());
    let mut cnt: usize = 1;

    for i in 0..total {
        if &this_tree == &trees[i] {
            cnt += 1;
        } else {
            println!("{} {:.4}", &this_tree, cnt as f32 / total as f32);
            this_tree = trees[i].clone();
            cnt = 1;
        }

        if i == total - 1 {
            println!("{} {:.4}", &this_tree, cnt as f32 / total as f32);
        }
    }


}