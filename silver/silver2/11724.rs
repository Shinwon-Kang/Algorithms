use std::io;
use std::vec;

fn dfs(idx: usize, edge: &Vec<Vec<usize>>, visit: &mut Vec<bool>) {
    visit[idx] = true;

    for i in edge[idx].iter() {
        if visit[*i] == false {
            dfs(*i, edge, visit);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();

    let n = split.next().unwrap().trim().parse::<usize>().unwrap();
    let m = split.next().unwrap().trim().parse::<usize>().unwrap();

    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut visit = vec![false; n + 1];
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        split = input.split_whitespace();

        let a = split.next().unwrap().trim().parse::<usize>().unwrap();
        let b = split.next().unwrap().trim().parse::<usize>().unwrap();

        edge[a].push(b);
        edge[b].push(a);
    }

    let mut cnt = 0;
    for idx in 1..(n + 1) {
        if visit[idx] == false {
            cnt += 1;

            dfs(idx, &edge, &mut visit);
        }
    }

    println!("{}", cnt);
}
