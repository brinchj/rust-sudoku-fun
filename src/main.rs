#![feature(iterator_step_by)]

#[macro_use]
extern crate chomp;
#[macro_use]
extern crate failure;
extern crate im;

use std::io::{self, Read};

mod board;
mod parser;

fn main() {
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    let mut buf = Vec::new();
    lock.read_to_end(&mut buf).unwrap();

    if let Ok(board) = parser::parse(&buf[..]) {
        println!("{:?}", board);

        if let Some(solved) = board.solve() {
            println!("input-board (valid={}):", board.validate());
            println!("{}", board.pretty());

            println!("solved (valid={}):", solved.validate());
            println!("{}", solved.pretty());

            assert!(board.validate());
            assert!(solved.solved());
        }
    }
}
