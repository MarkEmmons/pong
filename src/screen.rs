use console::Term;

use crate::ecs::{Board, Score, Position};

pub const SCREEN_X: usize = 75;
pub const SCREEN_Y: usize = 25;

pub const SCREEN_MID_X: usize = SCREEN_X / 2;
pub const SCREEN_MID_Y: usize = SCREEN_Y / 2;

pub type Screen = [[char; SCREEN_X]; SCREEN_Y];

// Populate the 2d array representing the screen with initial values
pub fn init_screen(board: &Board) -> Screen {

	let mut screen: Screen = [['*'; SCREEN_X]; SCREEN_Y];

	for r in 1..(screen.len() - 1) {

		for c in 1..(screen[r].len() - 1) {

			screen[r][c] = if c == SCREEN_MID_X {
				'|'
			} else {
				' '
			};
		}
	}


	let zip = board
		.score_components
		.iter()
		.zip(board.position_components.iter());

	let player_paddles =
		zip.filter_map(|(score, position): (&Option<Score>, &Option<Position>)| {
			Some((score.as_ref()?, position.as_ref()?))
		});

	for(score, position) in player_paddles {

		// Draw Paddle
		screen[position.pos_y - 1][position.pos_x] = '|';
		screen[position.pos_y][position.pos_x] = '|';
		screen[position.pos_y + 1][position.pos_x] = '|';

		// Draw Score
		let score_pos: usize = if position.pos_x < SCREEN_MID_X {
			SCREEN_MID_X / 2
		} else {
			(SCREEN_MID_X / 2) * 3
		};

		let scores: Vec<char> = score.0.to_string().chars().collect();
		screen[0][score_pos-1] = ' ';
		screen[0][score_pos] = scores[0];
		screen[0][score_pos+1] = ' ';
	}

	screen
}

pub fn draw_screen(screen: Screen, stdout: &Term) {

	let _result = stdout.clear_screen();
	for row in screen.iter() {

		let _result = stdout.write_line(row
			.iter()
			.collect::<String>()
			.as_str()
		);
	}
}

pub fn update_screen(screen: &mut Screen, board: &Board) {

	for r in 1..(screen.len() - 1) {

		for c in 1..(screen[r].len() - 1) {

			screen[r][c] = if c == SCREEN_MID_X {
				'|'
			} else {
				' '
			};
		}
	}


	let zip = board
		.score_components
		.iter()
		.zip(board.position_components.iter());

	let player_paddles =
		zip.filter_map(|(score, position): (&Option<Score>, &Option<Position>)| {
			Some((score.as_ref()?, position.as_ref()?))
		});

	for(score, position) in player_paddles {

		// Draw Paddle
		screen[position.pos_y - 1][position.pos_x] = '|';
		screen[position.pos_y][position.pos_x] = '|';
		screen[position.pos_y + 1][position.pos_x] = '|';

		// Draw Score
		let score_pos: usize = if position.pos_x < SCREEN_MID_X {
			SCREEN_MID_X / 2
		} else {
			(SCREEN_MID_X / 2) * 3
		};

		let scores: Vec<char> = score.0.to_string().chars().collect();
		screen[0][score_pos-1] = ' ';
		screen[0][score_pos] = scores[0];
		screen[0][score_pos+1] = ' ';
	}
}
