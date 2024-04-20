use std::fmt::Write;
use std::io;

fn search_area(board: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    let mut path: Vec<(usize, usize)> = vec![(i, j)];
    let mut ss = 1;
    while !path.is_empty() {
        let tmp = path.pop().unwrap();
        if board[tmp.0 - 1][tmp.1] {
            ss += 1;
            board[tmp.0 - 1][tmp.1] = false;
            path.push((tmp.0 - 1, tmp.1));
        }
        if board[tmp.0 + 1][tmp.1] {
            ss += 1;
            board[tmp.0 + 1][tmp.1] = false;
            path.push((tmp.0 + 1, tmp.1));
        }
        if board[tmp.0][tmp.1 - 1] {
            ss += 1;
            board[tmp.0][tmp.1 - 1] = false;
            path.push((tmp.0, tmp.1 - 1));
        }
        if board[tmp.0][tmp.1 + 1] {
            ss += 1;
            board[tmp.0][tmp.1 + 1] = false;
            path.push((tmp.0, tmp.1 + 1));
        }
    }

    ss
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = line[0] + 2; // 5
    let m = line[1] + 2; // 7

    let mut board: Vec<Vec<bool>> = vec![vec![true; m]; n];
    for i in 0..m {
        board[0][i] = false;
        board[n - 1][i] = false;
    }
    for i in 0..n {
        board[i][0] = false;
        board[i][m - 1] = false;
    }

    for _ in 0..line[2] {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        for i in line[0]..line[2] {
            for j in line[1]..line[3] {
                board[j + 1][i + 1] = false;
            }
        }
    }

    let mut cnt: u32 = 0;
    let mut ss: Vec<u32> = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if board[i][j] {
                cnt += 1;
                board[i][j] = false;
                ss.push(search_area(&mut board, i, j));
            }
        }
    }
    ss.sort();

    let mut output = String::new();
    for ele in ss {
        write!(output, "{} ", ele).unwrap();
    }

    println!("{}", cnt);
    println!("{}", output.trim());
}
