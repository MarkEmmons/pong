mod ecs;
mod input;
mod screen;

use console::Term;

use crate::ecs::Board;
use crate::screen::Screen;

fn main() {

	// Initialize Terminal
	let stdout = Term::stdout();

	// Initialize Board
	let mut board = Board::new();
	Board::init_board(&mut board);

	// Initialize Screen
	let screen: Screen = screen::init_screen(&board);

	loop {

		screen::draw_screen(screen, &stdout);

		input::get_user_input(&stdout);

		screen::update_screen(&screen, &board);
	}
}
