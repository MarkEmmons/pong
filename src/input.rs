use log::info;
use console::{Key, Term};

use crate::constants::{UP, DOWN};
use crate::Board;

pub fn get_user_input(terminal: &Term, board: &mut Board, player1: usize, player2: usize) {

	if let Ok(key) = terminal.read_key() {

		match key {

			// Supported Keys - Up and Down
			Key::ArrowUp => {
				board.move_entity(player2, UP);
			}
			Key::ArrowDown => {
				board.move_entity(player2, DOWN);
			}

			// Character Keys
			Key::Char(c) => {
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
			Key::ArrowLeft => info!("Caught unused key: Left"),
			Key::ArrowRight => info!("Caught unused key: Right"),
			Key::Enter => info!("Caught unused key: Enter"),
			Key::Escape => info!("Caught unused key: Escape"),
			Key::Backspace => info!("Caught unused key: Backspace"),
			Key::Home => info!("Caught unused key: Home"),
			Key::End => info!("Caught unused key: End"),
			Key::Tab => info!("Caught unused key: Tab"),
			Key::BackTab => info!("Caught unused key: BackTab"),
			Key::Alt => info!("Caught unused key: Alt"),
			Key::Del => info!("Caught unused key: Del"),
			Key::Shift => info!("Caught unused key: Shift"),
			Key::Insert => info!("Caught unused key: Insert"),
			Key::PageUp => info!("Caught unused key: PageUp"),
			Key::PageDown => info!("Caught unused key: PageDown"),

			_ => info!("Caught unused key: Couldn't match key"),
		}
	}

	match board.move_autos(player1, player2) {
		Some(player) => board.update_score(player),
		_ => (),
	}
}
