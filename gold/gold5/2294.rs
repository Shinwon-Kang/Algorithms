use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();
    let n = line[0];
    let k = line[1];

    let mut coins: Vec<u32> = Vec::new();
    let mut dp = vec![10001; 10001];

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let coin = input.trim().parse::<u32>().unwrap();

        if coin > 10000 || dp[coin as usize] == 1 {
            continue;
        }

        dp[coin as usize] = 1;
        coins.push(coin);
    }

    for i in 1..k + 1 {
        if dp[i] == 1 {
            continue;
        }

        for coin in coins.iter() {
            let left: i32 = i as i32 - *coin as i32;
            if left < 0 {
                continue;
            }

            dp[i] = if dp[i] > dp[left as usize] + 1 {
                dp[left as usize] + 1
            } else {
                dp[i]
            }
        }
    }

    if dp[k] == 10001 {
        println!("-1");
        return;
    }
    println!("{}", dp[k]);
}
