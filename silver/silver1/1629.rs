use std::io;
use::std::os::raw::c_ulonglong;

fn modular(a: u64, b: u64, c: u64) -> c_ulonglong {
    if b == 1 {
        return (a % c) as c_ulonglong;
    }

    if b % 2 == 0 {
        let m = modular(a, b/2, c);
        return (m * m) % c;
    } else {
        return (modular(a, b-1, c) * (a % c)) % c;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let a = iter.next().unwrap().trim().parse::<_>().unwrap();
    let b = iter.next().unwrap().trim().parse::<_>().unwrap();
    let c = iter.next().unwrap().trim().parse::<_>().unwrap();

    println!("{}", modular(a, b, c));
}
