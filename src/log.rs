use std::error::Error;
use std::process::exit;

use super::CMD;

fn print_err(msg: &str, err: &dyn Error) {
    eprintln!("{}: [ERR] {}, err = {}", CMD, msg, err);
}

pub fn err_exit(msg: &str, err: &dyn Error, code: i32) -> ! {
    print_err(msg, err);
    exit(code);
}
