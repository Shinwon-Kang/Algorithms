use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    let mut tree: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        let n1 = line[0];
        let n2 = line[1];

        tree[n1].push(n2);
        tree[n2].push(n1);
    }

    let mut par = vec![-1; n + 1];
    par[1] = 0;

    let mut queue = vec![1];
    while !queue.is_empty() {
        let node = queue.pop().unwrap();

        for child in tree[node].iter() {
            if par[*child] == -1 {
                par[*child] = node as i32;
                queue.push(*child);
            }
        }
    }

    let mut output = String::new();
    for i in 2..n + 1 {
        writeln!(output, "{}", par[i]).unwrap();
    }

    print!("{}", output);
}
