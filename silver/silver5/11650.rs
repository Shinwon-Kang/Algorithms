use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();
    let mut list: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<i32>>();
        list.push((line[0], line[1]));
    }

    list.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut output = String::new();
    for coord in list.into_iter() {
        writeln!(output, "{} {}", coord.0, coord.1).unwrap();
    }

    println!("{}", output);
}
