use std::collections::VecDeque;
use std::{cmp::min, io};

fn main() {
    let u32_max: u32 = 987654321;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m = input.trim().parse::<usize>().unwrap();

    let mut bus = vec![vec![u32_max; n + 1]; n + 1];
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        bus[line[0]][line[1]] = min(bus[line[0]][line[1]], line[2] as u32);
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    if line[0] == line[1] {
        println!("0");
        return;
    }

    let mut visit = vec![false; n + 1];
    visit[0] = true;
    visit[line[0]] = true;

    let mut will_visit: VecDeque<(usize, u32)> = VecDeque::new();
    for i in 1..n + 1 {
        if !visit[i] && bus[line[0]][i] != u32_max {
            will_visit.push_back((i, bus[line[0]][i]));
        }
    }

    will_visit.make_contiguous().sort_by(|a, b| a.1.cmp(&b.1));
    while !will_visit.is_empty() {
        let index = will_visit.pop_front().unwrap().0;
        if visit[index] {
            continue;
        }

        visit[index] = true;
        for i in 1..n + 1 {
            if !visit[i] && bus[index][i] != u32_max {
                bus[line[0]][i] = min(bus[line[0]][i], bus[line[0]][index] + bus[index][i]);
                will_visit.push_back((i, bus[line[0]][i]));
            }
        }
        will_visit.make_contiguous().sort_by(|a, b| a.1.cmp(&b.1));
    }

    println!("{}", bus[line[0]][line[1]]);
}

