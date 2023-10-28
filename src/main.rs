use std::collections::HashSet;
use std::io::{self, BufRead};

fn get_prio(c: char) -> u64 {
    let val = c as u64;
    // println!("Val: {}: {}", c, val);
    if val >= ('a' as u64) {
        val - ('a' as u64) + 1
    } else {
        val - ('A' as u64) + 27
    }
}
fn main() {
    // println!("{}", 'a' as u64);
    let mut total_common_prio = 0u64;
    let mut h: HashSet<char> = HashSet::new();
    let mut common: HashSet<char> = HashSet::new();
    let mut remaining_group = 3;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        remaining_group -= 1;

        let _ = match remaining_group {
            2 => {
                common = HashSet::new();
                let _ = line
                    .chars()
                    .map(|c| common.insert(c))
                    .collect::<Vec<bool>>();
            }
            0 => {
                h.clear();
                let _ = line.chars().map(|c| h.insert(c)).collect::<Vec<bool>>();
                common = common.intersection(&h).map(|&c| c).collect();
                assert_eq!(common.len(), 1);
                total_common_prio += get_prio(common.drain().next().unwrap());
                common.clear();
                remaining_group = 3;
            }
            _ => {
                h.clear();
                let _ = line.chars().map(|c| h.insert(c)).collect::<Vec<bool>>();
                common = common.intersection(&h).map(|&c| c).collect();
            }
        };
    }
    println!("Common Prio Total: {}", total_common_prio);
}
