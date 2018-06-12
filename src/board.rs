use im::{self, HashMap};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn all() -> impl Iterator<Item = Position> {
        (1..=9).flat_map(|y| (1..=9).map(move |x| Position { x, y }))
    }
    pub fn rows() -> impl Iterator<Item = impl Iterator<Item = Position>> {
        (0..=8).map(|row| Self::all().skip(9 * row).take(9))
    }
    pub fn cols() -> impl Iterator<Item = impl Iterator<Item = Position>> {
        (0..=8).map(|col| Self::all().skip(col).step_by(9))
    }
    pub fn boxs() -> impl Iterator<Item = impl Iterator<Item = Position>> {
        [1, 4, 7].iter().flat_map(move |y| {
            [1, 4, 7].iter().map(move |x| {
                Self::all().filter(move |p| *x <= p.x && p.x <= x + 2 && *y <= p.y && p.y <= y + 2)
            })
        })
    }
    pub fn next(&self) -> Option<Self> {
        if self.x == 9 && self.y == 9 {
            None
        } else {
            Some({
                if self.x == 9 {
                    Position {
                        x: 1,
                        y: self.y + 1,
                    }
                } else {
                    Position {
                        x: self.x + 1,
                        y: self.y,
                    }
                }
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    inner: HashMap<Position, u8>,
}

impl Board {
    pub fn new(input: impl Iterator<Item = (Position, Option<u8>)>) -> Board {
        Board {
            inner: input
                .filter_map(move |(pos, opt)| opt.map(|n| (pos, n)))
                .collect(),
        }
    }
    pub fn pretty(&self) -> String {
        let mut s = String::new();
        for pos in Position::all() {
            s.push_str(&format!("{} ", self.at(&pos).unwrap_or(0)));
            if pos.x == 9 {
                s.push_str("\n");
            }
        }
        s
    }
    pub fn at(&self, pos: &Position) -> Option<u8> {
        self.inner.get(&pos).map(|x| *x)
    }
    pub fn with(&self, pos: Position, value: u8) -> Option<Board> {
        assert!(1 <= value && value <= 9);
        assert!(1 <= pos.x && pos.x <= 9);
        assert!(1 <= pos.y && pos.y <= 9);

        let board = Board {
            inner: self.inner.insert(pos, value),
        };

        if board.validate_at(pos) {
            Some(board)
        } else {
            None
        }
    }
    fn line_dups(&self, line: impl Iterator<Item = Position>) -> usize {
        let values = line.flat_map(|pos| self.at(&pos)).collect::<Vec<_>>();
        values.len() - values.iter().collect::<HashSet<_>>().len()
    }
    fn validate_at(&self, pos: Position) -> bool {
        let col = Position::cols()
            .skip(pos.x as usize - 1)
            .next()
            .expect("BUG: x out of bounds");

        let row = Position::rows()
            .skip(pos.y as usize - 1)
            .next()
            .expect("BUG: y out of bounds");

        let bx_idx = (pos.x - 1) / 3 + (pos.y - 1) / 3 * 3;
        let bx = Position::boxs()
            .skip(bx_idx as usize)
            .next()
            .expect("BUG: box out of bounds");

        self.line_dups(col) == 0 && self.line_dups(row) == 0 && self.line_dups(bx) == 0
    }
    pub fn validate(&self) -> bool {
        Position::cols().all(|col| self.line_dups(col) == 0)
            && Position::rows().all(|row| self.line_dups(row) == 0)
            && Position::boxs().all(|bx| self.line_dups(bx) == 0)
    }
    pub fn solved(&self) -> bool {
        Position::all().filter(|x| self.at(x).is_none()).count() == 0
    }
    fn solve_inner(&self, pos: Position, line: im::OrdSet<u8>) -> Option<Board> {
        let solve_next = |board: &Board, value: u8| match pos.next() {
            Some(npos) if npos.y == pos.y => board.solve_inner(npos, line.remove(&value)),
            Some(npos) => board.solve_inner(npos, (1..=9).collect()),
            None => Some(board.clone()),
        };

        if let Some(n) = self.at(&pos) {
            solve_next(self, n)
        } else {
            line.iter()
                .filter_map(move |n| self.with(pos, *n).and_then(|b| solve_next(&b, *n)))
                .next()
        }
    }
    pub fn solve(&self) -> Option<Board> {
        if self.solved() {
            if self.validate() {
                Some(self.clone())
            } else {
                None
            }
        } else {
            self.solve_inner(Position { x: 1, y: 1 }, (1..=9).collect())
        }
    }
}
