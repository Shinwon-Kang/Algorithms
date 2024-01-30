use std::io;

fn func(x1: f64, x2: f64, x3: f64, y1: f64, y2: f64, y3: f64) -> f64 {
    let tmp = (x1 * y2 + x2 * y3 + x3 * y1) - (x2 * y1 + x3 * y2 + x1 * y3);
    tmp * 0.5
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let std_coord = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<f64>>();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut prev_coord = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<f64>>();

    let mut sum = 0.0;
    for _ in 2..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let curr_coord = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<f64>>();

        let tmp = func(
            std_coord[0],
            prev_coord[0],
            curr_coord[0],
            std_coord[1],
            prev_coord[1],
            curr_coord[1],
        );

        sum += tmp;
        prev_coord = curr_coord;
    }

    println!("{:.1}", sum.abs());
}
