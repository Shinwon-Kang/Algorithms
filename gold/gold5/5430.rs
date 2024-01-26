use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse::<usize>().unwrap();

    let mut output = String::new();
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim().to_string();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim().parse::<usize>().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums = input.trim().to_string();
        let nums = nums[1..nums.len() - 1].to_string();

        let mut deque: VecDeque<u16> = VecDeque::new();

        if s != 0 {
            for c in nums.split(',') {
                deque.push_back(c.trim().parse::<u16>().unwrap());
            }
        }

        let mut bk = false;
        let mut flag = true;
        for c in cmd.chars() {
            if c == 'R' {
                flag = !flag;
            } else if c == 'D' {
                if deque.is_empty() {
                    writeln!(output, "error").unwrap();
                    bk = true;
                    break;
                }
                if flag {
                    deque.pop_front();
                } else {
                    deque.pop_back();
                }
            }
        }

        if !bk && !deque.is_empty() {
            write!(output, "[").unwrap();
            if flag {
                for i in 0..deque.len() - 1 {
                    write!(output, "{},", deque[i]).unwrap();
                }
                writeln!(output, "{}]", deque[deque.len() - 1]).unwrap();
            } else {
                for i in (1..deque.len()).rev() {
                    write!(output, "{},", deque[i]).unwrap();
                }
                writeln!(output, "{}]", deque[0]).unwrap();
            }
        }

        if !bk && deque.is_empty() {
            writeln!(output, "[]").unwrap();
        }
    }
    println!("{}", output);
}
