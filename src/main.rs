use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ptr::null;

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

struct FileLine {
    name: String,
    num: u64,
    line: String,
}

fn readline_surround_of_file(
    name: &str,
    line_num: u64,
    before_line_count: u64,
    after_line_count: u64,
) -> Vec<FileLine> {
    let file = File::open(name);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            panic!("error {:?}", e)
        }
    };

    let cap = (before_line_count + 1 + after_line_count) as usize;
    let mut vec: Vec<FileLine> = Vec::with_capacity(cap);
    let num_range = (line_num - before_line_count)..(line_num + after_line_count);
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(f) => f,
            Err(e) => {
                panic!("error {:?}", e)
            }
        };
        let i = i as u64;
        if num_range.contains(&i) {
            let fl = FileLine {
                name: name.to_string(),
                num: i,
                line: line,
            };
            vec.push(fl);
        }
    }
    vec
}
