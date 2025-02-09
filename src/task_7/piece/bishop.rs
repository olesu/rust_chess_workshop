use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{MoveDirection, Square};

#[derive(Clone)]
pub struct Bishop {
    color: Color,
    position: (u8, u8),
}

const BISHOP_NAME: &str = "laupar";

impl Bishop {
    pub fn get_south_east_diagonal(position: &(u8, u8)) -> Vec<(u8, u8)> {
        let sum = position.0 + position.1;
        match sum {
            0 => vec![(0, 0)],
            1 => vec![(0, 1), (1, 0)],
            2 => vec![(0, 2), (1, 1), (2, 0)],
            3 => vec![(0, 3), (1, 2), (2, 1), (3, 0)],
            4 => vec![(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)],
            5 => vec![(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)],
            6 => vec![(0, 6), (1, 5), (2, 4), (3, 3), (4, 2), (5, 1), (6, 0)],
            7 => vec![(0, 7), (1, 6), (2, 5), (3, 4), (4, 3), (5, 2), (6, 1), (7, 0)],
            8 => vec![(1, 7), (2, 6), (3, 5), (4, 4), (5, 3), (6, 2), (7, 1)],
            9 => vec![(2, 7), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2)],
            10 => vec![(3, 7), (4, 6), (5, 5), (6, 4), (7, 3)],
            11 => vec![(4, 7), (5, 6), (6, 5), (7, 4)],
            12 => vec![(5, 7), (6, 6), (7, 5)],
            13 => vec![(6, 7), (7, 6)],
            14 => vec![(7, 7)],
            _ => panic!()
        }
    }

    pub fn get_north_east_diagonal(position: &(u8, u8)) -> Vec<(u8, u8)> {
        let difference = position.1 as i8 - position.0 as i8;
        match difference {
            7 => vec![(0, 7)],
            6 => vec![(0, 6), (1, 7)],
            5 => vec![(0, 5), (1, 6), (2, 7)],
            4 => vec![(0, 4), (1, 5), (2, 6), (3, 7)],
            3 => vec![(0, 3), (1, 4), (2, 5), (3, 6), (4, 7)],
            2 => vec![(0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7)],
            1 => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)],
            0 => vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
            -1 => vec![(1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5), (7, 6)],
            -2 => vec![(2, 0), (3, 1), (4, 2), (5, 3), (6, 4), (7, 5)],
            -3 => vec![(3, 0), (4, 1), (5, 2), (6, 3), (7, 4)],
            -4 => vec![(4, 0), (5, 1), (6, 2), (7, 3)],
            -5 => vec![(5, 0), (6, 1), (7, 2)],
            -6 => vec![(6, 0), (7, 1)],
            -7 => vec![(7, 0)],
            _ => panic!()
        }
    }
}

impl Piece for Bishop {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Bishop {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♝',
            Color::Black => '♗',
        }
    }
    fn get_name(&self) -> String {
        String::from(BISHOP_NAME)
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> &(u8, u8) {
        &self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        todo!()
    }
}