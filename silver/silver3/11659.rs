use io::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut split = input.split_whitespace();

    let n = split.next().unwrap().trim().parse::<u32>().unwrap();
    let m = split.next().unwrap().trim().parse::<u32>().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let mut s = 0;
    let sum: Vec<u32> = input
        .split_whitespace()
        .map(|x| {
            s += x.trim().parse::<u32>().unwrap();
            s
        })
        .collect();

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        split = input.split_whitespace();

        let i = split.next().unwrap().parse::<usize>().unwrap();
        let j = split.next().unwrap().parse::<usize>().unwrap();

        let mut res = sum[j - 1];
        if i > 1 {
            res -= sum[i - 2];
        }

        writeln!(out, "{}", res).unwrap();
    }
}
