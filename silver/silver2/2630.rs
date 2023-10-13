use std::io;

fn search(n: usize, i: usize, j: usize, board: &Vec<Vec<i16>>) -> (u16, u16) {
    if n == 1 {
        if board[i][j] == 0 {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }

    let mut s: i16 = 0;
    for _i in i..i + n {
        for _j in j..j + n {
            s += board[_i][_j];
        }
    }

    if s == 0 {
        return (1, 0);
    } else if s == ((n as i16) * (n as i16)) {
        return (0, 1);
    }

    let r1 = search(n / 2, i, j, board);
    let r2 = search(n / 2, i, j + n / 2, board);
    let r3 = search(n / 2, i + n / 2, j, board);
    let r4 = search(n / 2, i + n / 2, j + n / 2, board);

    (r1.0 + r2.0 + r3.0 + r4.0, r1.1 + r2.1 + r3.1 + r4.1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut board: Vec<Vec<i16>> = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        board.push(
            input
                .split_whitespace()
                .into_iter()
                .map(|x| x.trim().parse().unwrap())
                .collect(),
        );
    }

    let cnt = search(n, 0, 0, &board);
    println!("{}\n{}", cnt.0, cnt.1);
}

