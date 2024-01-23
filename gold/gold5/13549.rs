use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let s = line[0];
    let e = line[1];
    let mut visit = vec![false; 100001];
    let mut q = vec![(s, 0)];
    visit[s] = true;
    while !q.is_empty() {
        let tmp = q.pop().unwrap();
        let p = tmp.0;
        let t = tmp.1;

        if p == e {
            println!("{}", t);
            break;
        }

        if p * 2 <= 100000 && !visit[p * 2] {
            q.push((p * 2, t));
            visit[p * 2] = true;
        }
        if p as i32 - 1 >= 0 && !visit[p - 1] {
            q.push((p - 1, t + 1));
            visit[p - 1] = true;        
        }
        if p + 1 <= 100000 && !visit[p + 1] {
            q.push((p + 1, t + 1));
            visit[p + 1] = true;
        }

        q.sort_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
    }
}
