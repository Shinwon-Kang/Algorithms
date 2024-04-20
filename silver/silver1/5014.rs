use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let f = line[0];
    let s = line[1];
    let g = line[2];
    let u = line[3];
    let d = line[4];

    let mut board = vec![1000001; f as usize];
    let mut path = VecDeque::from([s - 1]);

    board[(s - 1) as usize] = 0;
    while !path.is_empty() {
        let tmp = path.pop_front().unwrap();

        if tmp + u < f && board[(tmp + u) as usize] > board[tmp as usize] + 1 {
            board[(tmp + u) as usize] = board[tmp as usize] + 1;
            path.push_back(tmp + u);
        }

        if tmp - d >= 0 && board[(tmp - d) as usize] > board[tmp as usize] + 1 {
            board[(tmp - d) as usize] = board[tmp as usize] + 1;
            path.push_back(tmp - d);
        }
    }

    let res = board[(g - 1) as usize];
    if res == 1000001 {
        println!("use the stairs");
    } else {
        println!("{}", res);
    }
}
