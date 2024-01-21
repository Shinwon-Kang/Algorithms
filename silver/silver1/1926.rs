use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let n = line[0];
    let m = line[1];

    let mut visit = vec![vec![false; m + 2]; n + 2];
    let mut board: Vec<Vec<u8>> = vec![vec![0; m + 2]; n + 2];
    let mut will_visit: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<u8>>();
        for j in 0..m {
            if line[j] == 1 {
                board[i + 1][j + 1] = 1;
                will_visit.push((i + 1, j + 1));
            }
        }
    }

    let mut cnt = 0;
    let mut max_size = 0;
    while !will_visit.is_empty() {
        let pp = will_visit.pop().unwrap();
        if visit[pp.0][pp.1] {
            continue;
        }
        
        visit[pp.0][pp.1] = true;
        let mut pic_size = 1;
        let mut q = vec![pp];
        while !q.is_empty() {
            let p = q.pop().unwrap();

            if board[p.0 - 1][p.1] == 1 && !visit[p.0 - 1][p.1] {
                q.push((p.0 - 1, p.1));
                visit[p.0 - 1][p.1] = true;
                pic_size += 1;
            }
            if board[p.0 + 1][p.1] == 1 && !visit[p.0 + 1][p.1] {
                q.push((p.0 + 1, p.1));
                visit[p.0 + 1][p.1] = true;
                pic_size += 1;
            }
            if board[p.0][p.1 - 1] == 1 && !visit[p.0][p.1 - 1] {
                q.push((p.0, p.1 - 1));
                visit[p.0][p.1 - 1] = true;
                pic_size += 1;
            }
            if board[p.0][p.1 + 1] == 1 && !visit[p.0][p.1 + 1] {
                q.push((p.0, p.1 + 1));
                visit[p.0][p.1 + 1] = true;
                pic_size += 1;
            }
        }
        cnt += 1;
        if max_size < pic_size {
            max_size = pic_size;
        }
    }

    println!("{}", cnt);
    println!("{}", max_size);
}
