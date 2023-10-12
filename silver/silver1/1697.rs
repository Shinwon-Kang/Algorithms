use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut split = input.split_whitespace();

    let n = split.next().unwrap().parse::<i32>().unwrap();
    let k = split.next().unwrap().parse::<i32>().unwrap();

    let mut que = VecDeque::from([(n, 0)]);
    let mut visit = vec![false; 100001];
    loop {
        let t = que.pop_front().unwrap();
        if k == t.0 {
            println!("{}", t.1);
            break;
        }

        if t.0 - 1 >= 0 && !visit[(t.0 as usize) - 1] {
            que.push_back((t.0 - 1, t.1 + 1));
        }
        if t.0 + 1 < 100001 && !visit[(t.0 as usize) + 1] {
            que.push_back((t.0 + 1, t.1 + 1));
        }
        if t.0 * 2 < 100001 && !visit[(t.0 as usize) * 2] {
            que.push_back((t.0 * 2, t.1 + 1));
        }

        visit[t.0 as usize] = true;
    }
}
