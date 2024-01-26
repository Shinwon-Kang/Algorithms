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
    let mut dp: Vec<u32> = vec![1; n];

    let mut max_cnt = 1;
    for i in 1..n {
        let s = list[i];
        let mut cnt = 0;
        for j in (0..i).rev() {
            if s > list[j] && cnt < dp[j] {
                cnt = dp[j];
            }
        }

        dp[i] = cnt + 1;
        
        max_cnt = if dp[i] > max_cnt {
            dp[i]
        } else {
            max_cnt
        }
    }

    println!("{}", max_cnt);
}