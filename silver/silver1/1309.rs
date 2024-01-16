use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse::<usize>().unwrap();
    if t == 1 {
        println!("3");
    } else if t == 2 {
        println!("7");
    } else {
        let mut dp: Vec<u64> = vec![0; t + 1];
        dp[1] = 2;
        dp[2] = 4;

        for i in 3..t + 1 {
            dp[i] = (2 + dp[i - 1] + dp[i - 2] * 2) % 9901;
            dp[i - 1] = (dp[i - 1] + dp[i - 2]) % 9901;
        }
        let result: u64 = (dp[t] + dp[t - 1] + 1) % 9901;
        println!("{}", result);
    }
}