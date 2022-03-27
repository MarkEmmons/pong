use log::info;

use pancurses::Input;

use crate::constants::{UP, DOWN};
use crate::Board;

pub fn get_user_input(terminal: &Term, board: &mut Board, player1: usize, player2: usize) {

	if let Some(key) = terminal.getch() {

		match key {

			// Supported Keys - Up and Down
			Input::KeyUp => {
				board.move_entity(player2, UP);
			}
			Input::KeyDown => {
				board.move_entity(player2, DOWN);
			}

			// Character Keys
			Input::Character(c) => {
				match c {

					// Quit when the user hits 'q'
					'q' => todo!("quit"),

					'w' => board.move_entity(player1, UP),

					's' => board.move_entity(player1, DOWN),

					// All other character keys unused
					_ => info!("Caught unused key: {}", c),
				}
			}

			// Special Keys not used
			Input::KeyLeft => info!("Caught unused key: Left"),
			Input::KeyRight => info!("Caught unused key: Right"),
			Input::KeyEnter => info!("Caught unused key: Enter"),
			Input::KeyBreak => info!("Caught unused key: Escape"),
			Input::KeyBackspace => info!("Caught unused key: Backspace"),
			Input::KeyHome => info!("Caught unused key: Home"),
			Input::KeyEnd => info!("Caught unused key: End"),
			Input::KeyTab => info!("Caught unused key: Tab"),
			Input::KeyBackTab => info!("Caught unused key: BackTab"),
			Input::KeyAlt => info!("Caught unused key: Alt"),
			Input::KeyDel => info!("Caught unused key: Del"),
			Input::KeyShift => info!("Caught unused key: Shift"),
			Input::KeyInsert => info!("Caught unused key: Insert"),
			Input::PageUp => info!("Caught unused key: PageUp"),
			Input::PageDown => info!("Caught unused key: PageDown"),

			_ => info!("Caught unused key: Couldn't match key"),
		}
	}

	match board.move_autos(player1, player2) {
		Some(player) => board.update_score(player),
		_ => (),
	}
}
