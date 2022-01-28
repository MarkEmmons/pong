mod ecs;
mod screen;

use console::Term;

use crate::ecs::Board;
use crate::screen::Screen;

fn main() {

	// Initialize Board
	let mut board = Board::new();
	screen::init_board(&mut board);

	// Initialize Screen
	let screen: Screen = screen::init_screen(&board);
	let stdout = Term::stdout();

	loop {

		screen::draw_screen(screen, &stdout);
		screen::get_user_input(&stdout);
		screen::update_screen(&screen, &board);
	}
}
