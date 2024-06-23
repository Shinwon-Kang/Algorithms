use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let array_size = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a_array = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<i32>>();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b_array = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<i32>>();

    let mut a_idx = 0;
    let mut b_idx = 0;

    let mut output = String::new();
    for _ in 0..(array_size[0] + array_size[1]) {
        if a_idx == array_size[0] {
            write!(output, "{} ", b_array[b_idx]).unwrap();
            b_idx += 1;
        } else if b_idx == array_size[1] {
            write!(output, "{} ", a_array[a_idx]).unwrap();
            a_idx += 1;
        } else if a_array[a_idx] < b_array[b_idx] {
            write!(output, "{} ", a_array[a_idx]).unwrap();
            a_idx += 1;
        } else {
            write!(output, "{} ", b_array[b_idx]).unwrap();
            b_idx += 1;
        }
    }

    println!("{}", output.trim());
}
