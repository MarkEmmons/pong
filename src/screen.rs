use console::Term;

use crate::components::{Position, Score, Trajectory};
use crate::constants::{SCREEN_X, SCREEN_Y, SCREEN_MID_X};
use crate::ecs::Board;

pub type Screen = [[char; SCREEN_X]; SCREEN_Y];

//// Public \\\\

// Populate the 2d array representing the screen with initial values
pub fn init_screen(board: &Board) -> Screen {

	let mut screen: Screen = [['*'; SCREEN_X]; SCREEN_Y];

	clear_board(&mut screen);

	update_paddles(&mut screen, board);

	update_balls(&mut screen, board);

	screen
}

// Update the array values with new positions
pub fn update_screen(screen: &mut Screen, board: &mut Board) {

	clear_board(screen);

	update_paddles(screen, board);

	update_balls(screen, board);
}

// Clear screen and draw current array
pub fn draw_screen(screen: Screen, terminal: &Term) {

	let _result = terminal.clear_screen();
	for row in screen.iter() {

		let _result = terminal.write_line(row
			.iter()
			.collect::<String>()
			.as_str()
		);
	}
}

//// Private \\\\

// Leave '*' on border, fill remaining cells with spaces and '|' in the center
fn clear_board(screen: &mut Screen) {

	for r in 1..(screen.len() - 1) {

		for c in 1..(screen[r].len() - 1) {

			screen[r][c] = if c == SCREEN_MID_X {
				'|'
			} else {
				' '
			};
		}
	}
}

// Update the position of each paddle on the screen array
fn update_paddles(screen: &mut Screen, board: &Board) {

	let mut positions = board
		.borrow_component_vec_mut::<Position>().unwrap();

	let mut scores = board
		.borrow_component_vec_mut::<Score>().unwrap();

	let zip = positions.iter_mut().zip(scores.iter_mut());
	let iter = zip.filter_map(| (position, score) | Some((position.as_mut()?, score.as_mut()?)));

	for(position, score) in iter {

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

// Update the position of each ball on the screen array
fn update_balls(screen: &mut Screen, board: &Board) {

	let mut positions = board
		.borrow_component_vec_mut::<Position>().unwrap();

	let mut trajectories = board
		.borrow_component_vec_mut::<Trajectory>().unwrap();

	let zip = positions.iter_mut().zip(trajectories.iter_mut());
	let iter = zip.filter_map(| (position, trajectory) | Some((position.as_mut()?, trajectory.as_mut()?)));

	for(position, _trajectory) in iter {

		// Draw Ball
		screen[position.pos_y][position.pos_x] = 'O';
	}
}
