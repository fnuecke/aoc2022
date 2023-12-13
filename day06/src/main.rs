use std::{fs::File, io::{self, Read}, collections::{VecDeque, HashMap}};

fn decr_or_del(counts: &mut HashMap<u8, usize>, key: &u8) {
    let counter = counts.get_mut(key).unwrap();
    if *counter == 1 {
        counts.remove(key);
    } else {
        *counter -= 1;
    }
}

fn add_or_incr(counts: &mut HashMap<u8, usize>, key: &u8) {
    *counts.entry(*key).or_insert(0) += 1;
}

fn compute(n: usize) -> usize {
    let file = File::open("input.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut window: VecDeque<u8> = VecDeque::with_capacity(n);

    let mut count: usize = 0;
    let mut counts = HashMap::new();
    loop {
        let mut buf: [u8; 1] = [0];
        match reader.read(&mut buf) {
            Ok(1) => {
                count += 1;

                // Ensure we don't exceed our window size (make room for new entry if needed).
                if window.len() == n {
                    let old_ch = window.pop_front().unwrap();
                    decr_or_del(&mut counts, &old_ch);
                }

                // Push new entry to window and count tracker.
                let ch = buf[0];
                window.push_back(ch);
                add_or_incr(&mut counts, &ch);

                // If window has reached saturation, check if all items are unique.
                if window.len() == n && counts.values().all(|v| *v == 1) {
                    break;
                }
            },
            _ => break,
        }        
    }

    count
}

fn main() {
    println!("part 1 = {}", compute(4));
    println!("part 2 = {}", compute(14));
}
