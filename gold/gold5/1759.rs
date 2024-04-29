use std::io;
use std::fmt::Write;

fn rec(passwd_length: usize, passwd: String, idx: usize, chars: &Vec<String>) -> Vec<String> {
    if passwd.len() == passwd_length {
        let mut cnt = 0;
        for s in passwd.chars() {
            if s == 'a' || s == 'e' || s == 'i' || s == 'o' || s == 'u' {
                cnt += 1;
            }
        }

        if cnt > 0 && passwd_length - cnt > 1 {
            return vec![passwd];
        }
    }

    let mut vec: Vec<String> = Vec::new();
    for i in idx..chars.len() {
        let mut new_passwd = passwd.clone();
        new_passwd += chars[i].as_str();

        vec.append(&mut rec(passwd_length, new_passwd, i + 1, &chars));
    }

    vec
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let line = input
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut chars = input
        .split_whitespace()
        .map(|x| x.trim().parse::<String>().unwrap())
        .collect::<Vec<_>>();
    chars.sort();

    let res = rec(line[0], String::from(""), 0, &chars);
    let mut output = String::new();
    for val in res.into_iter() {
        writeln!(output, "{}", val).unwrap();
    }
    print!("{}", output);
}
