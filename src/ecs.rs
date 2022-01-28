use crate::screen::{SCREEN_X, SCREEN_MID_X, SCREEN_MID_Y};

pub struct Score(pub u8);
pub struct Position(pub usize, pub usize);

pub struct Board {
	pub score_components: Vec<Option<Score>>,
	pub position_components: Vec<Option<Position>>,
}

impl Board {

	pub fn new() -> Self {
		Self {
			score_components: Vec::new(),
			position_components: Vec::new(),
		}
	}

	pub fn new_entity(&mut self, score: Option<Score>, position: Option<Position>) {
		self.score_components.push(score);
		self.position_components.push(position);
	}

	pub fn init_board(board: &mut Board) {

		// Player 1
		board.new_entity(
			Some(Score(0)),
			Some(Position(
				SCREEN_MID_Y,
				1,
			)));

		// Player 2
		board.new_entity(
			Some(Score(0)),
			Some(Position(
				SCREEN_MID_Y,
				SCREEN_X - 2,
			)));

		// Ball
		board.new_entity(
			None,
			Some(Position(
				SCREEN_MID_Y,
				SCREEN_MID_X,
			)));
	}
}
