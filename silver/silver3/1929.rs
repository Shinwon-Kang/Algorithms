use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let m = line[0];
    let n = line[1];

    if n == 1 {
        return;
    }

    let mut output = String::new();
    let mut primes = vec![true; n + 1];
    for i in 2..n + 1 {
        if primes[i] {
            if i >= m {
                writeln!(output, "{}", i).unwrap();
            }

            let mut cnt = 2;
            while i * cnt < n + 1 {
                primes[i * cnt] = false;
                cnt += 1;
            }
        }
    }

    println!("{}", output);
}
