use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut map: Vec<Vec<u8>> = vec![];
    let mut path: Vec<Vec<u64>> = vec![vec![0; n]; n];
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        map.push(
            input
                .split_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect::<Vec<_>>(),
        )
    }

    path[0][0] = 1;
    for i in 0..n {
        for j in 0..n {
            let m = map[i][j] as usize;
            if m == 0 || ((i != 0 || j != 0) && path[i][j] == 0) {
                continue;
            }

            if i + m < n {
                path[i + m][j] += path[i][j];
            }

            if j + m < n {
                path[i][j + m] += path[i][j];
            }
        }
    }
    println!("{}", path[n - 1][n - 1]);
}