use std::io;

fn main() {
    let mut list = vec![0; 42];

    let mut input = String::new();
    for _ in 0..10 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let tmp = input.trim().parse::<usize>().unwrap() % 42;
        list[tmp] += 1;
    }

    let mut cnt = 0;
    for i in list.into_iter() {
        if i != 0 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
