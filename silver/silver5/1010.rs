use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let ss = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();
        let n = ss[0];
        let m = ss[1];

        let mut dp = vec![vec![0; m]; n];
        dp[n - 1][m - 1] = 1;
        for i in ((n - 1)..m - 1).rev() {
            dp[n - 1][i] = dp[n - 1][i + 1] + 1;
        }

        for i in (0..n - 1).rev() {
            for j in (i..(i + (m - n + 1))).rev() {
                dp[i][j] = dp[i + 1][j + 1] + dp[i][j + 1];
            }
        }

        println!("{}", dp[0][0]);
    }
}