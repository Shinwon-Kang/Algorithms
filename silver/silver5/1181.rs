use std::fmt::Write;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    let mut list: Vec<String> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let s = input.clone().trim().to_string();
        list.push(s);
    }

    list.sort_by(|a, b| if a.len() != b.len() {
        a.len().cmp(&b.len())
    } else {
        a.cmp(&b)
    });

    
    let mut output = String::new();
    writeln!(output, "{}", list[0]).unwrap();
    let mut tmp = &list[0];
    for i in 1..n {
        if *tmp == list[i] {
            continue;
        }
        writeln!(output, "{}", list[i]).unwrap();
        tmp = &list[i];
    }

    print!("{}", output);
}
