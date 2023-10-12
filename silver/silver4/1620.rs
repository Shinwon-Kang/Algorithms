use io::Write;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut split = input.split_whitespace();

    let n: u32 = split.next().unwrap().parse().unwrap();
    let m: u32 = split.next().unwrap().parse().unwrap();

    let mut s_pokemons = Vec::new();
    let mut pokemons = HashMap::new();
    for idx in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        pokemons.insert(input.trim().to_owned(), idx + 1);
        s_pokemons.push(input.trim().to_owned());
    }

    let mut output = io::BufWriter::new(io::stdout().lock());
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<usize>() {
            Ok(_idx) => {
                writeln!(output, "{}", s_pokemons[_idx-1]);
            }
            Err(_) => {
                writeln!(output, "{}", pokemons.get(&input.trim().to_owned()).unwrap());
            }
        };
    }
}
