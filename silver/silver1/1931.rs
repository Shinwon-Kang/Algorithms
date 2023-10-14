use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut table = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut split = input.split_whitespace();

        let s_time = split.next().unwrap().trim().parse::<u32>().unwrap();
        let e_time = split.next().unwrap().trim().parse::<u32>().unwrap();

        table.push((e_time, s_time));
    }
    table.sort();

    let mut cnt = 1;
    let mut curr_e = table[0].0;

    for (e, s) in &table[1..] {
        if curr_e <= *s {
            cnt += 1;

            curr_e = *e;
        }
    }

    println!("{}", cnt);
}


