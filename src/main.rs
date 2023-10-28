use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let mut stacks: Vec<VecDeque<char>> = vec![];
    let mut init = false;
    let mut broken = false;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        if !broken {
            if line.len() == 0 {
                broken = true
            }
            if !init {
                let stack_count = (line.len() + 1) / 4;
                for _ in 0..stack_count {
                    stacks.push(VecDeque::new());
                }
                init = true;
            }

            let mut prev_is_open = false;

            for (index, c) in line.char_indices() {
                if prev_is_open {
                    stacks[(index - 1) / 4].push_front(c);
                    prev_is_open = false;
                } else if c == '[' {
                    prev_is_open = true
                } else {
                    prev_is_open = false;
                }
            }
        } else {
            let move_nums: Vec<usize> = line.split(" ").filter_map(|n| n.parse().ok()).collect();
            assert_eq!(move_nums.len(), 3);
            let [a, b, c] = move_nums[..3] else {panic!("Not the right number of moves")};
            for _ in 0..a {
                let val = stacks[b - 1].pop_back().unwrap();
                stacks[c - 1].push_back(val);
            }
        }
    }
    let res: Vec<_> = stacks.iter().filter_map(|v| v.back()).collect();
    let res: String = res.into_iter().collect();
    println!("{:?}", res);
}
