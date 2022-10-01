mod cli;
mod linestring;

use std::fs::File;
use std::io::{self, BufRead};

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    let (before_context, after_context) = args.adjust();
    let mut before_file_name = String::new();
    let mut line_nums: Vec<u64> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(f) => f,
            Err(e) => {
                panic!("error {:?}", e)
            }
        };
        let l = linestring::parse_line_prefix(line);
        let l = match l {
            Ok(f) => f,
            Err(e) => {
                panic!("error {:?}", e)
            }
        };
        let file_name = l.0;
        let line_num: u64 = l.1;
        line_nums.push(line_num);

        if before_file_name == "" {
            before_file_name = file_name.to_string();
            continue;
        }

        // reading lines when file name was changed.
        if before_file_name == file_name {
            continue;
        }

        run(&before_file_name, &line_nums, before_context, after_context);
        before_file_name = file_name.to_string();
        line_nums = vec![];
    }

    if before_file_name != "" {
        run(&before_file_name, &line_nums, before_context, after_context);
    }
}

fn run(file_name: &String, line_nums: &Vec<u64>, before_context: u64, after_context: u64) {
    let file = File::open(&file_name);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => {
            // TODO
            panic!("error {:?}", e)
        }
    };

    let lines = linestring::readline_surround_of_line_number(
        &mut file,
        &line_nums,
        before_context,
        after_context,
    );
    let lines = match lines {
        Ok(f) => f,
        Err(e) => {
            panic!("error {:?}", e)
        }
    };
    for (_, line) in lines {
        println!("{}:{}:{}", &file_name, line.line_num, line.line);
    }
}
