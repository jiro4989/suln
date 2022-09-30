use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(f) => f,
            Err(e) => {
                panic!("error {:?}", e)
            }
        };
        let cols: Vec<&str> = line.split(":").collect();
        let file_name = cols[0];
        let line_num: usize = cols[1].parse().unwrap();
        let file = File::open(file_name);
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                panic!("error {:?}", e)
            }
        };
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(f) => f,
                Err(e) => {
                    panic!("error {:?}", e)
                }
            };
            if i == line_num {
                print!("found! num = {}, str = {}", i, line);
            }
        }
    }
}
