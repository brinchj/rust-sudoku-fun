#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate sudokusolver;

fuzz_target!(|data: &[u8]| {
    if let Ok(board) = sudokusolver::parser::parse(data) {
        if let Some(solved) = board.solve() {
            println!("input-board (valid={}):", board.validate());
            println!("{}", board.pretty());

            println!("solved (valid={}):", solved.validate());
            println!("{}", solved.pretty());

            assert!(board.validate());
            assert!(solved.validate());
            assert!(solved.solved());
        }
    }
});
