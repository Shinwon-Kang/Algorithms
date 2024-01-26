use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    let mut dp: Vec<u64> = vec![1, 1, 1, 2, 2];
    for i in 5..101 {
        dp.push(dp[i - 1] + dp[i - 5]);
    }


    let mut output = String::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let t = input.trim().parse::<usize>().unwrap();

        writeln!(output, "{}", dp[t - 1]).unwrap();
    }
    println!("{}", output);
}
