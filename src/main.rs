mod parsing;
mod display;
mod engine;
mod write_output;

use std::fmt::Display;
use crate::engine::find_optimal_paths;
use crate::parsing::{parse_game, PathType};

fn main() {

    println!("Hello, world!");
    let mut game = parse_game("01-comedy");
    find_optimal_paths(&mut game);

    println!("{}", game);

    write_output::write_output(&game);
}


#[derive(Debug, Clone, PartialEq)]
struct PlacedPath {
    id: PathType,
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn move_x(&self, x: u16) -> u16 {
        match self {
            Direction::Right => x + 1,
            Direction::Left => x - 1,
            _ => x,
        }
    }
    pub fn move_y(&self, y: u16) -> u16 {
        match self {
            Direction::Up => y - 1,
            Direction::Down => y + 1,
            _ => y,
        }
    }
    pub fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
