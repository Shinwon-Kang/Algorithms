use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();
    let mut dp: Vec<u64> = vec![0; 91];
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..n + 1 {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    println!("{}", dp[n]);
}
