use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    let mut output = io::BufWriter::new(io::stdout().lock());
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut split = input.split_whitespace();
        
        let a = split.next().unwrap().trim().parse().unwrap();
        let b = split.next().unwrap().trim().parse().unwrap();
        let c = split.next().unwrap().trim().parse().unwrap();

        if a == 0 && b == 0 && c == 0 {
            break;
        }

        let mut vec: Vec<u32> = vec![a, b, c];
        vec.sort();

        if (vec[0] * vec[0]) + (vec[1] * vec[1]) == (vec[2] * vec[2]) {
            writeln!(output, "right").unwrap();
        } else {
            writeln!(output, "wrong").unwrap();
        }
    }
}
