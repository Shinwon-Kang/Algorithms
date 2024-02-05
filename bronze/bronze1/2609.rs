use std::{cmp::min, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>();

    let mut c = 0;
    for i in 1..min(line[0], line[1]) + 1 {
        if line[0] % i == 0 && line[1] % i == 0 {
            c = i;
        }
    }

    let mut cnt = 1;
    let min_num = min(line[0], line[1]);
    loop {
        if (min_num * cnt) % line[0] == 0 && (min_num * cnt) % line[1] == 0 {
            break;
        }

        cnt += 1;
    }

    println!("{}", c);
    println!("{}", min_num * cnt);
}
