use std::{io, cmp::min};

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
    let mut dp: Vec<Vec<u32>> = vec![vec![0; n + 1]; n + 1];

    for i in 1..n + 1 {
        dp[1][i] = prices[0] + dp[1][i - 1];
    }
    for i in 2..n + 1 {
        for j in 1..n + 1 {
            if i > j {
                dp[i][j] = dp[i - 1][j];
            } else {
                let a = dp[i - 1][j];
                let b = prices[i - 1] + dp[i][j - i];
                dp[i][j] = min(a, b);
            }
        }
    }

    println!("{}", dp[n][n]);
}
