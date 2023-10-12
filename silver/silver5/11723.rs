use io::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();

    let mut output = io::BufWriter::new(io::stdout().lock());

    let mut s: [bool; 21] = [false; 21];
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let mut split = input.split_whitespace();
        let cmd = split.next().unwrap();

        if cmd == "add" {
            let val = split.next().unwrap().trim().parse::<usize>().unwrap();
            s[val] = true;
        } else if cmd == "remove" {
            let val = split.next().unwrap().trim().parse::<usize>().unwrap();
            s[val] = false;
        } else if cmd == "check" {
            let val = split.next().unwrap().trim().parse::<usize>().unwrap();
            if s[val] {
                writeln!(output, "{}", 1).unwrap();
            } else {
                writeln!(output, "{}", 0).unwrap();
            }
        } else if cmd == "toggle" {
            let val = split.next().unwrap().trim().parse::<usize>().unwrap();
            s[val] = !s[val];
        } else if cmd == "all" {
            s = [true; 21];
        } else if cmd == "empty" {
            s = [false; 21];
        }
    }
}
