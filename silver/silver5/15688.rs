use std::fmt::Write;
use std::io;

fn merge(arr: Vec<i32>) -> Vec<i32> {
    let arr_size = arr.len();

    if arr_size == 1 {
        return arr;
    }

    let a = merge(arr[0..(arr_size / 2 as usize)].to_vec());
    let b = merge(arr[(arr_size / 2 as usize)..arr_size].to_vec());
    let mut output = Vec::new();

    let mut a_idx = 0;
    let mut b_idx = 0;

    for _ in 0..(a.len() + b.len()) {
        if a_idx == a.len() {
            output.push(b[b_idx]);
            b_idx += 1;
        } else if b_idx == b.len() {
            output.push(a[a_idx]);
            a_idx += 1;
        } else if a[a_idx] < b[b_idx] {
            output.push(a[a_idx]);
            a_idx += 1;
        } else {
            output.push(b[b_idx]);
            b_idx += 1;
        }
    }

    output
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_size = input.trim().parse::<usize>().unwrap();

    let mut nums = Vec::new();
    for _ in 0..input_size {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        nums.push(input.trim().parse::<i32>().unwrap());
    }

    let output_arr = merge(nums);

    let mut output = String::new();
    for val in output_arr {
        writeln!(output, "{}", val).unwrap();
    }
    
    println!("{}", output);
}
