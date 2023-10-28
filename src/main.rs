use std::io::{self, BufRead};

fn main() {
    let mut count = 0u64;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        let res: Vec<u64> = line
            .split(",")
            .flat_map(|s| s.split("-"))
            .filter_map(|n| n.parse().ok())
            .collect();

        assert_eq!(4, res.len());

        let _ = match res[..] {
            [a, b, x, y] => {
                if ((a <= x) && (b >= y)) || ((x <= a) && (y >= b)) {
                    count += 1;
                }
            }
            _ => unreachable!(),
        };
    }
    println!("Total Covered Assignemnts: {:?}", count);
}
