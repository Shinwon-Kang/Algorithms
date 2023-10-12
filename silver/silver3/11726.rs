use std::{io, vec};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num = input.trim().parse::<usize>().unwrap();
    let mut board: Vec<u64> = vec![0; 1001];

    let modular = 10007;

    board[1] = 1 % modular;
    board[2] = 2 % modular;

    for idx in 3..(num + 1) {
        board[idx] = ((board[idx - 1] % modular) + (board[idx - 2] % modular)) % modular;
    }

    println!("{}", board[num]);
}
