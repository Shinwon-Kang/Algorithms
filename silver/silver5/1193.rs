use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse::<usize>().unwrap();

    let mut level = 1;
    let mut pos = 1;


    let mut cnt = 1;
    loop {
        if x == cnt {
            break;
        }

        if pos == level {
            level += 1;
            pos = 1;
        } else {
            pos += 1;
        }

        cnt += 1;
    }

    if level % 2 == 1 {
        println!("{}/{}", level - (pos - 1), 1 + (pos - 1));
    } else {
        println!("{}/{}", 1 + (pos - 1), level - (pos - 1));
    }
}
