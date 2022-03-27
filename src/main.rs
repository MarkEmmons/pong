mod ecs;
mod chars;
mod constants;
mod components;
mod input;
mod screen;
mod systems;
mod logger;

use std::thread;
use std::time::Duration;
use log::info;

use pancurses::{Window};
use pancurses::{initscr, endwin, cbreak, noecho};

use crate::constants::{SCREEN_X, SCREEN_MID_X, SCREEN_MID_Y};
use crate::components::{Position, Trajectory, Score};
use crate::ecs::Board;
use crate::screen::Screen;

fn main() {

	// Initialize Logger
	logger::init_logger();

	// Initialize Terminal
	info!("Initializing Terminal...");
	let window: Window = initscr();

	cbreak();
	noecho();

	window.keypad(true);
	window.nodelay(true);

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
		trj_y: 0,
	});

	// Initialize Screen
	info!("Initializing Screen...");
	let mut screen: Screen = screen::init_screen(&board);

	let mut frame = 1;
	let mut quit = false;
	while !quit {

		screen::draw_screen(screen, &window);

		quit = input::get_user_input(&window, &mut board, player1, player2);

		if frame % 16 == 0 {

			match board.move_autos(player1, player2) {
				Some(player) => board.update_score(player),
				_ => (),
			}

			frame = 1;
		}

		screen::update_screen(&mut screen, &mut board);

		thread::sleep(Duration::from_millis(1));
		frame += 1;
	}
	endwin();
}
