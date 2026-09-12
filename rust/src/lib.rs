#![allow(unused_variables)]
#![allow(dead_code)]

pub mod args;
pub mod init;
pub mod object;

use args::parse;

pub fn run() {
    parse();
}
