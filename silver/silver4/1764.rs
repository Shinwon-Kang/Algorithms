use std::collections::HashSet;
use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();

    let n = split.next().unwrap().trim().parse::<u32>().unwrap();
    let m = split.next().unwrap().trim().parse::<u32>().unwrap();

    let mut set: HashSet<String> = HashSet::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        set.insert(input.to_owned());
    }

    let mut dupl: Vec<String> = Vec::new();
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if set.contains(&input) {
            dupl.push(input.trim().to_owned());
        }
    }
    dupl.sort();

    let mut output = io::BufWriter::new(io::stdout().lock());
    writeln!(output, "{}", dupl.len()).unwrap();

    for val in dupl {
        writeln!(output, "{}", val).unwrap();
    }
}

