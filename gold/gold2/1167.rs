use std::io;
use std::cmp::max;

fn rec(tree: &Vec<Vec<(usize, u32)>>, node: usize, dist: u32, visit: &mut Vec<bool>, max_dist: &mut u32) -> u32 {
    visit[node] = true;

    let mut child_dist: Vec<u32> = Vec::new();
    for childs in tree[node].iter() {
        if !visit[childs.0] {
            child_dist.push(rec(tree, childs.0, childs.1, visit, max_dist));
        }
    }

    if child_dist.is_empty() {
        return dist;
    }

    child_dist.sort();
    let node_dist = if child_dist.len() >= 2 {
        child_dist[child_dist.len() - 1] + child_dist[child_dist.len() - 2]
    } else {
        child_dist[0]
    };

    *max_dist = max(*max_dist, node_dist);

    return dist + child_dist[child_dist.len() - 1];
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let v = input.trim().parse::<usize>().unwrap();

    let mut tree: Vec<Vec<(usize, u32)>> = vec![vec![]; v + 1];
    for _ in 0..v {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<i32>>();

        let node = line[0] as usize;
        for i in (1..line.len() - 1).step_by(2) {
            let next = line[i] as usize;
            let dist = line[i + 1] as u32;

            tree[node].push((next, dist));
        }
    }

    let mut visit = vec![false; v + 1];
    visit[0] = true;

    let mut max_dist: u32 = 0;
    rec(&tree, 1, 0, &mut visit, &mut max_dist);

    println!("{}", max_dist);
}
