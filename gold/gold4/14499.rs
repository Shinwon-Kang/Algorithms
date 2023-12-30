use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut line = input
        .split_whitespace()
        .map(|x| x.trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let N = line[0] as usize;
    let M = line[1] as usize;
    let mut x = line[2];
    let mut y = line[3];
    let num_command = line[4];

    let mut map: Vec<Vec<u16>> = vec![vec![0; M]; N];
    for i in 0..N {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        line = input
            .split_whitespace()
            .map(|x| x.trim().parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        for j in 0..M {
            map[i][j] = line[j];
        }
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    line = input
        .split_whitespace()
        .map(|x| x.trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let mut dice: [u16; 6] = [0; 6];
    for cmd in line.into_iter() {
        if cmd == 1 && (y < M as u16 - 1) {
            dice = [dice[1], dice[5], dice[2], dice[0], dice[4], dice[3]];
            y += 1;
        } else if cmd == 2 && (y > 0) {
            dice = [dice[3], dice[0], dice[2], dice[5], dice[4], dice[1]];
            y -= 1;
        } else if cmd == 3 && (x > 0) {
            dice = [dice[2], dice[1], dice[5], dice[3], dice[0], dice[4]];
            x -= 1;
        } else if cmd == 4 && (x < N as u16 - 1) {
            dice = [dice[4], dice[1], dice[0], dice[3], dice[5], dice[2]];
            x += 1;
        } else {
            continue;
        }

        if map[x as usize][y as usize] == 0 {
            map[x as usize][y as usize] = dice[0];
        } else {
            dice[0] = map[x as usize][y as usize];
            map[x as usize][y as usize] = 0;
        }

        println!("{}", dice[5]);
    }
}
