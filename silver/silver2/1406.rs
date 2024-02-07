use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut v1 = input.trim().chars().map(|x| x).collect::<Vec<char>>();
    let mut v2: Vec<char> = Vec::new();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let m = line.trim().parse::<usize>().unwrap();

    for _ in 0..m {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();

        let cmd = line
            .split_whitespace()
            .map(|x| x.trim().chars().nth(0).unwrap())
            .collect::<Vec<char>>();

        match cmd[0] {
            'L' => {
                if !v1.is_empty() {
                    v2.push(v1.pop().unwrap());
                }
            }
            'D' => {
                if !v2.is_empty() {
                    v1.push(v2.pop().unwrap());
                }
            }
            'B' => {
                if !v1.is_empty() {
                    v1.pop();
                }
            }
            'P' => {
                v1.push(cmd[1]);
            }
            _ => (),
        }
    }

    v2.reverse();
    v1.append(&mut v2);
    println!("{}", v1.iter().collect::<String>())
}
