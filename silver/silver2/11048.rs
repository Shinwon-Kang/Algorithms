use std::cmp::max;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let n = line[0];
    let m = line[1];

    let mut dp: Vec<Vec<u32>> = vec![vec![0; m + 1]; n + 1];
    for i in 1..n + 1 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<u32>>();

        for j in 1..m + 1 {
            dp[i][j] = line[j - 1];
        }
    }

    for i in 1..n + 1 {
        for j in 1..m + 1 {
            dp[i][j] = dp[i][j] + max(dp[i - 1][j - 1], max(dp[i - 1][j], dp[i][j - 1]));
        }
    }

    println!("{}", dp[n][m]);
}
