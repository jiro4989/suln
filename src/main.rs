mod cli;
mod lineparser;
mod linestring;
mod log;

use std::fs::File;
use std::io::{self, BufRead};

use clap::Parser;
use cli::Cli;

use log::err_exit;

static CMD: &str = "suln";
const ERR_CODE_READ_STDIN: i32 = 1;
const ERR_CODE_PARSE_LINE_NUMBER: i32 = 2;
const ERR_CODE_OPEN_FILE: i32 = 3;
const ERR_CODE_READ_LINES: i32 = 4;

fn main() {
    let args = Cli::parse();
    let (before_context, after_context) = args.adjust();
    let mut before_file_name = String::new();
    let mut line_nums: Vec<u64> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(f) => f,
            Err(e) => err_exit("failed to read stdin.", &e, ERR_CODE_READ_STDIN),
        };
        let l = lineparser::parse(&line);
        if l.is_none() {
            continue;
        }
        let l = l.unwrap();
        let file_name = l.file_name;
        let line_num: u64 = l.line_num;
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
        Err(e) => err_exit("failed to open file.", &e, ERR_CODE_OPEN_FILE),
    };

    let lines = linestring::readline_surround_of_line_number(
        &mut file,
        &line_nums,
        before_context,
        after_context,
    );
    let lines = match lines {
        Ok(f) => f,
        Err(e) => err_exit("failed to raed lines.", &e, ERR_CODE_READ_LINES),
    };
    for (_, line) in lines {
        println!("{}:{}:{}", &file_name, line.line_num, line.line);
    }
}
