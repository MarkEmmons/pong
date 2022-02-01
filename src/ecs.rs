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

pub enum Entity {
	Player(Option<Position>, Option<Score>),
	Ball(Option<Position>),
}

pub struct Board {
	pub entity_components: Vec<Entity>,
}

impl Board {

	pub fn new() -> Self {
		Self {
			entity_components: Vec::new(),
		}
	}

	pub fn new_entity(&mut self, position: Option<Position>, score: Option<Score>) {
		match score {
			None => self.entity_components.push(Entity::Ball(position)),
			Some(ref _s) => self.entity_components.push(Entity::Player(position, score)),
		}
	}

	pub fn init_board(board: &mut Board) {

		// Player 1
		board.new_entity(
			Some(Position {
				pos_x: 1,
				pos_y: SCREEN_MID_Y,
			}),
			Some(Score(0))
		);

		// Player 2
		board.new_entity(
			Some(Position {
				pos_x: SCREEN_X - 2,
				pos_y: SCREEN_MID_Y,
			}),
			Some(Score(0))
		);

		// Ball
		board.new_entity(
			Some(Position {
				pos_x: SCREEN_MID_X,
				pos_y: SCREEN_MID_Y,
			}),
			None
		);
	}

	pub fn move_player(&mut self, player: usize, direction: isize) {

		info!("Moving player {}: {}", player, direction);

		// Get current y position, will return 0 if position is None
		let cur_pos_y = match &self.entity_components[player] {
			Entity::Player(player, _score) => {
				player
				.as_ref()
				.unwrap_or(&INVALID_POSITION)
				.pos_y
			}
			_ => 0
		};

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
		match &mut self.entity_components[player] {
			Entity::Player(player, _score) => {
				player
				.as_mut()
				.unwrap()
				.pos_y = new_pos;
			}
			_ => (),
		}

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
