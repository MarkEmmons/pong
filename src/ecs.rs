use log::info;

use crate::screen::{SCREEN_X, SCREEN_Y, SCREEN_MID_X, SCREEN_MID_Y};

pub struct Position {
	pub pos_x: usize,
	pub pos_y: usize,
}

const INVALID_POSITION: Position = Position {
	pos_x: 0,
	pos_y: 0,
};

pub struct Score(pub u8);

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
			Some(Position {
				pos_x: 1,
				pos_y: SCREEN_MID_Y,
			}));

		// Player 2
		board.new_entity(
			Some(Score(0)),
			Some(Position {
				pos_x: SCREEN_X - 2,
				pos_y: SCREEN_MID_Y,
			}));

		// Ball
		board.new_entity(
			None,
			Some(Position {
				pos_x: SCREEN_MID_X,
				pos_y: SCREEN_MID_Y,
			}));
	}


	pub fn move_player(&mut self, player: usize, direction: isize) {

		info!("Moving player {}: {}", player, direction);

		// Get current y position, will return 0 if position is None
		let cur_pos_y = self.position_components[player]
			.as_ref()
			.unwrap_or(&INVALID_POSITION)
			.pos_y;

		// Invalid position -> unwrap returned NONE
		if cur_pos_y == 0 {
			info!("Couldn't find valid position for player {}", player);
			return;
		}

		let new_pos = match update_position(cur_pos_y, direction) {
			None => cur_pos_y,
			Some(new_pos_y) => new_pos_y,
		};

		// New position is out of bounds, don't update
		if new_pos <= 1
			|| new_pos >= SCREEN_Y-2 {
			info!("New position {} out of bounds", new_pos);
			return;
		}

		// Unwrap should be Some since we validated when copying pos_x
		self.position_components[player]
			.as_mut()
			.unwrap()
			.pos_y = new_pos;

		info!("Player position moved to {}", new_pos);
	}
}

fn update_position(pos: usize, dir: isize) -> Option<usize> {

	if dir.is_negative() {
		pos.checked_sub(dir.wrapping_abs() as usize)
	} else {
		pos.checked_add(dir as usize)
	}
}
