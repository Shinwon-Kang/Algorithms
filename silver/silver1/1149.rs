use std::io;
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u16>().unwrap();

    let mut cost: [u32; 3] = [0; 3];
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let prices: Vec<u32> = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<_>>();

        let mut new_cost = cost.clone();

        new_cost[0] = prices[0] + cmp::min(cost[1], cost[2]);
        new_cost[1] = prices[1] + cmp::min(cost[0], cost[2]);
        new_cost[2] = prices[2] + cmp::min(cost[0], cost[1]);

        cost = new_cost;
    }

    println!("{}", cmp::min(cmp::min(cost[0], cost[1]), cost[2]));
}

