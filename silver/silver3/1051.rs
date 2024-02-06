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

    let mut board: Vec<Vec<u8>> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let line = input
            .trim()
            .chars()
            .map(|x| x as u8 - 48)
            .collect::<Vec<u8>>();
        board.push(line);
    }

    let mut max_len = 1;
    for i in 0..n {
        for j in 0..m {
            let mut len = 1;

            while i + len < n && j + len < m {
                if (board[i][j] == board[i + len][j])
                    && (board[i][j] == board[i][j + len])
                    && (board[i][j] == board[i + len][j + len])
                {
                    if max_len < len + 1 {
                        max_len = len + 1;
                    }
                }
                len += 1;
            }
        }
    }

    println!("{}", max_len.pow(2));
}
