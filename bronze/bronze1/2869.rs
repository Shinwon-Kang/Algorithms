use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>();

    let day_up = line[0] - line[1];
    let goal = line[2] - line[0];

    let c = (goal as f64 / day_up as f64).ceil() + 1.0;

    println!("{}", c);
}
