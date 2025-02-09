use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::MoveDirection;

/// # Oppgave 7
///
/// I denne oppgaven skal vi implementere trekkene til dronningen. Du finner metodene som skal
/// implementeres `impl Piece for Queen {}`-blokken. (Se etter en `todo!()`.
///
/// Dronningen kan bevege seg så mange trekk den vil i alle retninger, det vil si nord, vest, sør og
/// øst i tilegg til diagonalene. Den kan derimot ikke hoppe over brikker slik som springeren.
///
/// Se [hint.md](../hint.md) for hint.
#[derive(Clone)]
pub struct Queen {
    pub color: Color,
    pub position: (u8, u8),
}

const QUEEN_NAME: &str = "dronning";

impl Piece for Queen {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Queen {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♛',
            Color::Black => '♕',
        }
    }
    fn get_name(&self) -> String {
        String::from(QUEEN_NAME)
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_7::piece::queen::Queen;

    #[test]
    fn test_queen_moves_1() {
        let queen = Queen::new(Color::White, "a1".as_u8().unwrap());
        let legal_moves = set![
            "a2", "a3", "a4", "a5", "a6", "a7", "a8",
            "b1", "c1", "d1", "e1", "f1", "g1", "h1",
            "b2", "c3", "d4", "e5", "f6", "g7", "h8"
        ];
        assert_eq_set!(queen.get_moves(&empty_set!(), &empty_set!()), legal_moves);
    }

    #[test]
    fn test_queen_moves_2() {
        let queen = Queen::new(Color::White, "d4".as_u8().unwrap());
        let legal_moves = set![
            "c4", "b4", "a4",
            "c5", "b6", "a7",
            "d5", "d6", "d7", "d8",
            "e5", "f6", "g7", "h8",
            "e4", "f4", "g4", "h4",
            "e3", "f2", "g1",
            "d3", "d2",  "d1",
            "c3", "b2", "a1"
        ];
        assert_eq_set!(queen.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }
}
