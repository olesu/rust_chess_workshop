use std::collections::HashSet;
use crate::color::Color;
use crate::piece::Piece;
use crate::square::{Square, Squares};

const PAWN_NAME: &str = "bonde";

#[derive(Clone)]
pub struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Pawn {
    pub(crate) fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8().unwrap();
        let moves: HashSet::<(i8, i8)> = match self.color {
            Color::White if self.position.0 == 1 => HashSet::from_iter([(2, x), (3, x)]),
            Color::White => HashSet::from_iter([(y + 1, x)]),
            Color::Black if self.position.0 == 6 => HashSet::from_iter([(5, x), (4, x)]),
            Color::Black => HashSet::from_iter([(y - 1, x)]),
        };
        moves.as_board_positions()
    }
    pub(crate) fn get_pawn_capture_moves(&self) -> HashSet<(u8, u8)> {
        // TODO: Add possible en passant captures
        let (y, x) = self.position.as_i8().unwrap();
        let capture_moves: HashSet<(i8, i8)> = match self.color {
            Color::White => HashSet::from_iter([(y + 1, x - 1), (y + 1, x + 1)]),
            Color::Black => HashSet::from_iter([(y - 1, x - 1), (y - 1, x + 1)]),
        };
        capture_moves.as_board_positions()
    }
}

impl Piece for Pawn {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Pawn {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }
    fn get_name(&self) -> String {
        String::from(PAWN_NAME)
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
        let all_pieces = team.union(rival_team).cloned().collect();
        let moves: HashSet<(u8, u8)> = self.get_pawn_moves().difference(&all_pieces).cloned().collect();
        let capture_moves: HashSet<(u8, u8)> = self.get_pawn_capture_moves().intersection(rival_team).cloned().collect();
        moves.union(&capture_moves).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::piece::pawn::Pawn;
    use crate::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn two_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = ["e3", "e4"].as_board_positions();
        assert_eq!(pawn.get_pawn_moves(), legal_moves)
    }

    #[test]
    fn two_capture_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = ["d3", "f3"].as_board_positions();
        assert_eq!(pawn.get_pawn_capture_moves(), legal_moves)
    }
}