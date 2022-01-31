mod ecs;
mod input;
mod screen;
mod logger;

use log::info;
use console::Term;

use crate::ecs::Board;
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
	Board::init_board(&mut board);

	// Initialize Screen
	info!("Initializing Screen...");
	let mut screen: Screen = screen::init_screen(&board);

	loop {

		screen::draw_screen(screen, &stdout);

		input::get_user_input(&stdout, &mut board);

		screen::update_screen(&mut screen, &board);
	}
}
