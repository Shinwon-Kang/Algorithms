use std::{io, vec};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();
    let n = line[0];
    let m = line[1];

    let mut board: Vec<Vec<u32>> = vec![vec![0; n + 1]; n + 1];
    for i in 1..n + 1 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<u32>>();

        for j in 1..n + 1 {
            board[i][j] = board[i - 1][j] + board[i][j - 1] - board[i - 1][j - 1] + line[j - 1];
        }
    }

    let mut output = String::new();
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        let x0 = line[0];
        let y0 = line[1];
        let x1 = line[2];
        let y1 = line[3];

        let val = board[x1][y1] + board[x0 - 1][y0 - 1] - (board[x0 - 1][y1] + board[x1][y0 - 1]);
        writeln!(output, "{}", val).unwrap();
    }
    print!("{}", output);
}
