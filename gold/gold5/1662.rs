use std::{io, vec};

fn recu(zip: &Vec<char>, idx: &mut usize) -> u32 {
    let vec_size = zip.len();

    let mut cnt = 0;
    while *idx < vec_size {
        if zip[*idx] == ')' {
            return cnt;
        } else if zip[*idx] == '(' {
            let x = zip[*idx - 1] as u32 - '0' as u32;
            *idx += 1;

            let cnt_temp = recu(zip, idx);
            cnt = (cnt - 1) + x * cnt_temp;
        } else {
            cnt += 1;
        }
        *idx+=1;
    }

    cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let zip: Vec<char> = input.trim().chars().collect();

    let cnt = recu(&zip, &mut 0);
    println!("{}", cnt);
}
