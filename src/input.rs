use log::info;
use console::{Key, Term};

use crate::Board;

const UP: isize = -1;
const DOWN: isize = 1;

pub fn get_user_input(stdout: &Term, board: &mut Board) {

	if let Ok(key) = stdout.read_key() {

		match key {

			// Supported Keys - Up and Down
			Key::ArrowUp => {
				board.move_player(1, UP);
			}
			Key::ArrowDown => {
				board.move_player(1, DOWN);
			}

			// Character Keys
			Key::Char(c) => {
				match c {

					// Quit when the user hits 'q'
					'q' => todo!("quit"),

					'w' => board.move_player(0, UP),

					's' => board.move_player(0, DOWN),

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
}
