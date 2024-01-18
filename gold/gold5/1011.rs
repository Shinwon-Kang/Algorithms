use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let lines = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<u32>>();
        let mut s = lines[0] + 1;
        let e = lines[1];

        let mut cnt = 0;
        let mut speed = 1;
        loop {
            if s == e {
                break;
            }

            let up = ((speed + 1) * (speed + 2)) / 2;
            let keep = (speed * (speed + 1)) / 2;
            let down = ((speed - 1) * speed) / 2;

            if e - s >= up {
                speed += 1;
            } else if e - s >= keep {
            } else if e - s >= down {
                speed -= 1;
            }
            s += speed;
            cnt += 1;
        }

        println!("{}", cnt + 1);
    }
}
