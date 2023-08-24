use std::fmt::Write;
use std::io;
use std::io::Read;

fn main() {
    faster_output();
    input_all();
    split_integer_with_space();
    split_integer_without_space();
    split_string_with_space();
    split_string();
}

fn faster_output() {
    let mut output = String::new();

    writeln!(output, "{} + {} = {}", 1, 2, 1 + 2).unwrap();
    // ...

    // 프로그램 마지막에 출력
    print!("{}", output);
}

fn input_all() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input);
}

// input: "1 2 3 4 5 6 7"
// output: [1, 2, 3, 4, 5, 6, 7]
fn split_integer_with_space() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input reading error");
    let split_vec: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.trim().parse().expect("input parsing error"))
        .collect::<Vec<_>>();

    println!("{:?}", split_vec);
}

// input: "1234567"
// output: [1, 2, 3, 4, 5, 6, 7]
fn split_integer_without_space() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input reading error");
    let split_vec: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();

    println!("{:?}", split_vec);
}

// input: "Hello Rust"
// output: ["Hello", "Rust"]
fn split_string_with_space() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input reading error");
    let split_vec: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    println!("{:?}", split_vec);
}

// input: "hello"
// output: ['h', 'e', 'l', 'l', 'o']
fn split_string() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input reading error");
    let split_vec = input.trim().chars().collect::<Vec<char>>();

    println!("{:?}", split_vec);
}
