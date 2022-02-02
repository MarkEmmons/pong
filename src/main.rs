mod ecs;
mod input;
mod screen;
mod logger;

use log::info;
use console::Term;

use crate::screen::{SCREEN_X, SCREEN_MID_X, SCREEN_MID_Y};
use crate::ecs::{Board, Position, Trajectory, Score};
use crate::screen::Screen;

fn main() {

	// Initialize Logger
	logger::init_logger();

	// Initialize Terminal
	info!("Initializing Terminal...");
	let stdout = Term::stdout();

	// Initialize Board
	info!("Initializing Board...");
	let mut board = Board::new();

	// Player 1
	let player1 = board.new_entity();
	board.add_component_to_entity(player1, Position {
		pos_x: 1,
		pos_y: SCREEN_MID_Y,
	});
	board.add_component_to_entity(player1, Score(0));

	// Player 2
	let player2 = board.new_entity();
	board.add_component_to_entity(player2, Position {
		pos_x: SCREEN_X - 2,
		pos_y: SCREEN_MID_Y,
	});
	board.add_component_to_entity(player2, Score(0));

	// Ball
	let ball = board.new_entity();
	board.add_component_to_entity(ball, Position {
		pos_x: SCREEN_MID_X,
		pos_y: SCREEN_MID_Y,
	});
	board.add_component_to_entity(ball, Trajectory {
		trj_x: 1,
		trj_y: 1,
	});

	// Initialize Screen
	info!("Initializing Screen...");
	let mut screen: Screen = screen::init_screen(&board);

	loop {

		screen::draw_screen(screen, &stdout);

		input::get_user_input(&stdout, &mut board, player1, player2);

		screen::update_screen(&mut screen, &mut board);
	}
}
