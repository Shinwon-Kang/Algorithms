use std::collections::VecDeque;
use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();

    let n = split.next().unwrap().trim().parse().unwrap();
    let m = split.next().unwrap().trim().parse().unwrap();

    let mut s_f = false;
    let mut s_x = 0;
    let mut s_y = 0;

    let mut board: Vec<Vec<_>> = Vec::new();
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        split = input.split_whitespace();

        board.push(Vec::new());
        for j in 0..m {
            let v = split.next().unwrap().trim().parse::<u8>().unwrap();
            if v == 2 {
                s_f = true;
                s_x = i;
                s_y = j;
            }

            board[i].push(v);
        }
    }

    let mut visit = vec![vec![false; m]; n];
    let mut path = vec![vec![0; m]; n];
    let mut route = VecDeque::from([(s_x, s_y)]);

    if s_f {
        visit[s_x][s_y] = true;
        while let Some(r) = route.pop_front() {
            if (r.0 as i16) - 1 >= 0 {
                if !visit[r.0 - 1][r.1] && board[r.0 - 1][r.1] == 1 {
                    path[r.0 - 1][r.1] = path[r.0][r.1] + 1;
                    route.push_back((r.0 - 1, r.1));
                    visit[r.0 - 1][r.1] = true;
                }
            }
            if r.0 + 1 < n {
                if !visit[r.0 + 1][r.1] && board[r.0 + 1][r.1] == 1 {
                    path[r.0 + 1][r.1] = path[r.0][r.1] + 1;
                    route.push_back((r.0 + 1, r.1));
                    visit[r.0 + 1][r.1] = true;
                }
            }
            if (r.1 as i16) - 1 >= 0 {
                if !visit[r.0][r.1 - 1] && board[r.0][r.1 - 1] == 1 {
                    path[r.0][r.1 - 1] = path[r.0][r.1] + 1;
                    route.push_back((r.0, r.1 - 1));
                    visit[r.0][r.1 - 1] = true;
                }
            }
            if r.1 + 1 < m {
                if !visit[r.0][r.1 + 1] && board[r.0][r.1 + 1] == 1 {
                    path[r.0][r.1 + 1] = path[r.0][r.1] + 1;
                    route.push_back((r.0, r.1 + 1));
                    visit[r.0][r.1 + 1] = true;
                }
            }
        }
    }

    let mut output = io::BufWriter::new(io::stdout().lock());
    for i in 0..n {
        for j in 0..m {
            if !visit[i][j] && board[i][j] == 1 {
                write!(output, "-1 ").unwrap();
            } else {
                write!(output, "{} ", path[i][j]).unwrap();
            }
        }
        writeln!(output, "").unwrap();
    }
}
