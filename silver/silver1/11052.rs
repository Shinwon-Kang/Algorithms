use std::{cmp::max, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let prices = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>();
    let mut dp: Vec<u32> = vec![0; n + 1];

    for i in 1..n + 1 {
        dp[i] = prices[i - 1];
    }

    for i in 1..n + 1 {
        for j in i..n + 1 {
            dp[j] = max(dp[j], dp[j - i] + prices[i-1]);
        }
    }

    println!("{}", dp[n]);
}