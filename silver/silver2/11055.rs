use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let list = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>();

    let mut dp: Vec<u32> = vec![0; n];
    dp[0] = list[0];

    let mut max_sum = dp[0];
    for i in 1..n {
        dp[i] = list[i];
        for j in (0..i).rev() {
            if (list[i] > list[j]) && (dp[i] < dp[j] + list[i]) {
                dp[i] = dp[j] + list[i];
            }
        }

        max_sum = if max_sum < dp[i] { dp[i] } else { max_sum };
    }

    println!("{}", max_sum);
}
