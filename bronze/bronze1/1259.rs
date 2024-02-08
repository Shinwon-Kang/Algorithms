use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input.trim().chars().collect::<Vec<char>>();

        if line.len() == 1 && line[0] == '0' {
            break;
        }

        let mut res = "yes";
        let len = line.len();
        for i in 0..len / 2 {
            if line[i] != line[(len - 1) - i] {
                res = "no";
                break;
            }
        }
        writeln!(output, "{}", res).unwrap();
    }

    println!("{}", output);
}
