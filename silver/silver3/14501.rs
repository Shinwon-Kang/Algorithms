use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let N = input.trim().parse::<usize>().unwrap();

    let mut T = vec![0; N];
    let mut P = vec![0; N];
    for i in 0..N {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse::<_>().unwrap())
            .collect::<Vec<_>>();

        T[i] = line[0];
        P[i] = line[1];
    }

    let mut dp = vec![0; N+1];

    for i in (0..N).rev() {
        if i + T[i] <= N && P[i] + dp[i + T[i]] > dp[i + 1] {
            dp[i] = P[i] + dp[i + T[i]]
        } else {
            dp[i] = dp[i + 1];
        }
    }

    println!("{}", dp[0]);
}
