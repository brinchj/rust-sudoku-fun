use chomp::prelude::*;
use failure::Error;

use board::{Board, Position};

fn to_value(n: u8) -> Option<u8> {
    if n == 0 {
        None
    } else {
        Some(n)
    }
}

pub fn parse(s: &[u8]) -> Result<Board, Error> {
    let digit = |i| satisfy(i, |b| b'0' <= b && b <= b'9');
    let whitespace = |i| satisfy(i, |b| b' ' == b);

    let chars: Vec<u8> = parse_only(|i| sep_by(i, digit, whitespace), s)
        .map_err(|(_rest, err)| format_err!("failed to parse: {}", err))?;

    let values: Vec<Option<u8>> = chars.into_iter().map(|x| x - b'0').map(to_value).collect();

    if Position::all().count() != values.len() {
        bail!(
            "incorrect number of digits provided: got {}, wanted {}",
            values.len(),
            Position::all().count()
        );
    }

    Ok(Board::new(Position::all().zip(values)))
}
