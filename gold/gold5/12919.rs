use std::io;

fn recu2(start: &String, end: String) -> bool {
    if *start == end {
        return true;
    }
    if start.len() == end.len() {
        return false;
    }

    let b = end.as_bytes();

    let mut res1 = false;
    if *b.last().unwrap() == 'A' as u8 {
        let mut new_end = end.clone();
        new_end.pop();

        res1 = recu2(&start, new_end);
    }

    let mut res2 = false;
    if *b.first().unwrap() == 'B' as u8 {
        let mut new_end = end.clone().chars().rev().collect::<String>();
        new_end.pop();

        res2 = recu2(&start, new_end);
    }

    res1 || res2
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let start = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let end = input.trim().to_string();

    let res = recu2(&start, end);
    match res {
        true => println!("1"),
        false => println!("0"),
    }

}
