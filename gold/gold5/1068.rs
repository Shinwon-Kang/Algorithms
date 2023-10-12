use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse().unwrap();
    let mut table: Vec<Vec<u32>> = vec![vec![]; n];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let mut parent_table: Vec<u32> = Vec::new();
    for (_idx, _value) in input.split_whitespace().enumerate() {
        let parent = _value.parse::<i32>().unwrap();
        if parent == -1 {
            parent_table.push(_idx as u32);
            continue;
        }

        table[parent as usize].push(_idx as u32);
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let drop_node = input.trim().parse::<u32>().unwrap();

    let mut leaf_cnt = 0;
    let mut stack: VecDeque<u32> = VecDeque::from(parent_table);

    while !stack.is_empty() {
        let node = stack.pop_back().unwrap();

        if node == drop_node {
            continue;
        }

        if table[node as usize].is_empty() {
            leaf_cnt += 1;
        } else {
            if table[node as usize].len() == 1 && table[node as usize][0] == drop_node {
                leaf_cnt += 1;
            } else {
                for n in table[node as usize].iter() {
                    stack.push_back(*n);
                }
            }
        }
    }

    println!("{}", leaf_cnt);
}
