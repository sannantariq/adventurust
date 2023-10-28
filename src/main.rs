use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

#[derive(Debug)]
struct CharCounter {
    counts: [usize; 26],
}

impl CharCounter {
    fn new() -> Self {
        Self {
            counts: [0usize; 26],
        }
    }

    // fn len(&self) -> usize {
    //     self.counts.iter().fold(0, |v, w| v + w)
    // }

    fn is_unique(&self) -> bool {
        self.counts
            .iter()
            .map(|&n| n <= 1)
            .fold(true, |v, w| v && w)
    }

    fn add(&mut self, c: char) {
        let idx = (c as usize) - ('a' as usize);
        self.counts[idx] += 1;
    }

    fn remove(&mut self, c: char) -> Result<(), Box<dyn std::error::Error>> {
        let idx = (c as usize) - ('a' as usize);
        if self.counts[idx] == 0 {
            return Err("This character does not exist".into());
        } else {
            self.counts[idx] -= 1;
            Ok(())
        }
    }
}

fn main() {
    let mut counter = CharCounter::new();
    let maxlen = 14usize;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        let mut buf: VecDeque<char> = VecDeque::new();
        for (idx, c) in line.char_indices() {
            counter.add(c);
            buf.push_back(c);
            if buf.len() > maxlen {
                counter.remove(buf.pop_front().unwrap()).unwrap();
            }
            if buf.len() == maxlen && counter.is_unique() {
                println!("Found unique sequence ending at index: {}", idx);
                return;
            }
        }
    }
}
