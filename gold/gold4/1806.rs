use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>();

    let s = line[1];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>();

    let mut l = 0;
    let mut r = 0;

    let mut sum = nums[0];
    let mut min_cnt = nums.len() + 1;

    while l != nums.len() - 1 || r != nums.len() - 1 {
        if sum >= s && r - l < min_cnt {
            min_cnt = r - l;
        }

        if (l == r || sum < s) && r < nums.len() - 1 {
            r += 1;
            sum += nums[r];
        } else {
            sum -= nums[l];
            l += 1;
        }
    }

    if sum >= s && r - l < min_cnt {
        min_cnt = r - l;
    }

    if min_cnt == nums.len() + 1 {
        println!("0");
    } else {
        println!("{}", min_cnt + 1);
    }
}

