use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    if n == 1 {
        println!("1");
        return;
    }

    let mut v: VecDeque<u32> = (1..=n).collect();

    let mut cnt: u64 = 1;
    while v.len() > 1 {
        let x = u64::pow(cnt, 3);
        let mo = x % v.len() as u64;

        if mo == 0 {
            v.pop_back();
        } else {
            for _i in 0..mo-1 {
                let y = v.pop_front().unwrap();
                v.push_back(y);
            }

            v.pop_front();
        }

        cnt += 1;
    }

    println!("{}", v[0]);
}