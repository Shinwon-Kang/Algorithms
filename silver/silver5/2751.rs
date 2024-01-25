use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();
    let mut list = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let t = input.trim().parse::<i32>().unwrap();

        list.push(t);
    }
    list.sort();

    let mut output = String::new();
    for i in list.into_iter() {
        writeln!(output, "{}", i).unwrap();
    }

    println!("{}", output);
}
