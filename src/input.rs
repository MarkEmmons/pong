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
			Input::KeyCodeYes => info!("Caught unused key: KeyCodeYes"),
			Input::KeyBreak => info!("Caught unused key: KeyBreak"),
			Input::KeyLeft => info!("Caught unused key: KeyLeft"),
			Input::KeyRight => info!("Caught unused key: KeyRight"),
			Input::KeyHome => info!("Caught unused key: KeyHome"),
			Input::KeyBackspace => info!("Caught unused key: KeyBackspace"),
			Input::KeyF0 => info!("Caught unused key: KeyF0"),
			Input::KeyF1 => info!("Caught unused key: KeyF1"),
			Input::KeyF2 => info!("Caught unused key: KeyF2"),
			Input::KeyF3 => info!("Caught unused key: KeyF3"),
			Input::KeyF4 => info!("Caught unused key: KeyF4"),
			Input::KeyF5 => info!("Caught unused key: KeyF5"),
			Input::KeyF6 => info!("Caught unused key: KeyF6"),
			Input::KeyF7 => info!("Caught unused key: KeyF7"),
			Input::KeyF8 => info!("Caught unused key: KeyF8"),
			Input::KeyF9 => info!("Caught unused key: KeyF9"),
			Input::KeyF10 => info!("Caught unused key: KeyF10"),
			Input::KeyF11 => info!("Caught unused key: KeyF11"),
			Input::KeyF12 => info!("Caught unused key: KeyF12"),
			Input::KeyF13 => info!("Caught unused key: KeyF13"),
			Input::KeyF14 => info!("Caught unused key: KeyF14"),
			Input::KeyF15 => info!("Caught unused key: KeyF15"),
			Input::KeyDL => info!("Caught unused key: KeyDL"),
			Input::KeyIL => info!("Caught unused key: KeyIL"),
			Input::KeyDC => info!("Caught unused key: KeyDC"),
			Input::KeyIC => info!("Caught unused key: KeyIC"),
			Input::KeyEIC => info!("Caught unused key: KeyEIC"),
			Input::KeyClear => info!("Caught unused key: KeyClear"),
			Input::KeyEOS => info!("Caught unused key: KeyEOS"),
			Input::KeyEOL => info!("Caught unused key: KeyEOL"),
			Input::KeySF => info!("Caught unused key: KeySF"),
			Input::KeySR => info!("Caught unused key: KeySR"),
			Input::KeyNPage => info!("Caught unused key: KeyNPage"),
			Input::KeyPPage => info!("Caught unused key: KeyPPage"),
			Input::KeySTab => info!("Caught unused key: KeySTab"),
			Input::KeyCTab => info!("Caught unused key: KeyCTab"),
			Input::KeyCATab => info!("Caught unused key: KeyCATab"),
			Input::KeyEnter => info!("Caught unused key: KeyEnter"),
			Input::KeySReset => info!("Caught unused key: KeySReset"),
			Input::KeyReset => info!("Caught unused key: KeyReset"),
			Input::KeyPrint => info!("Caught unused key: KeyPrint"),
			Input::KeyLL => info!("Caught unused key: KeyLL"),
			Input::KeyAbort => info!("Caught unused key: KeyAbort"),
			Input::KeySHelp => info!("Caught unused key: KeySHelp"),
			Input::KeyLHelp => info!("Caught unused key: KeyLHelp"),
			Input::KeyBTab => info!("Caught unused key: KeyBTab"),
			Input::KeyBeg => info!("Caught unused key: KeyBeg"),
			Input::KeyCancel => info!("Caught unused key: KeyCancel"),
			Input::KeyClose => info!("Caught unused key: KeyClose"),
			Input::KeyCommand => info!("Caught unused key: KeyCommand"),
			Input::KeyCopy => info!("Caught unused key: KeyCopy"),
			Input::KeyCreate => info!("Caught unused key: KeyCreate"),
			Input::KeyEnd => info!("Caught unused key: KeyEnd"),
			Input::KeyExit => info!("Caught unused key: KeyExit"),
			Input::KeyFind => info!("Caught unused key: KeyFind"),
			Input::KeyHelp => info!("Caught unused key: KeyHelp"),
			Input::KeyMark => info!("Caught unused key: KeyMark"),
			Input::KeyMessage => info!("Caught unused key: KeyMessage"),
			Input::KeyMove => info!("Caught unused key: KeyMove"),
			Input::KeyNext => info!("Caught unused key: KeyNext"),
			Input::KeyOpen => info!("Caught unused key: KeyOpen"),
			Input::KeyOptions => info!("Caught unused key: KeyOptions"),
			Input::KeyPrevious => info!("Caught unused key: KeyPrevious"),
			Input::KeyRedo => info!("Caught unused key: KeyRedo"),
			Input::KeyReference => info!("Caught unused key: KeyReference"),
			Input::KeyRefresh => info!("Caught unused key: KeyRefresh"),
			Input::KeyReplace => info!("Caught unused key: KeyReplace"),
			Input::KeyRestart => info!("Caught unused key: KeyRestart"),
			Input::KeyResume => info!("Caught unused key: KeyResume"),
			Input::KeySave => info!("Caught unused key: KeySave"),
			Input::KeySBeg => info!("Caught unused key: KeySBeg"),
			Input::KeySCancel => info!("Caught unused key: KeySCancel"),
			Input::KeySCommand => info!("Caught unused key: KeySCommand"),
			Input::KeySCopy => info!("Caught unused key: KeySCopy"),
			Input::KeySCreate => info!("Caught unused key: KeySCreate"),
			Input::KeySDC => info!("Caught unused key: KeySDC"),
			Input::KeySDL => info!("Caught unused key: KeySDL"),
			Input::KeySelect => info!("Caught unused key: KeySelect"),
			Input::KeySEnd => info!("Caught unused key: KeySEnd"),
			Input::KeySEOL => info!("Caught unused key: KeySEOL"),
			Input::KeySExit => info!("Caught unused key: KeySExit"),
			Input::KeySFind => info!("Caught unused key: KeySFind"),
			Input::KeySHome => info!("Caught unused key: KeySHome"),
			Input::KeySIC => info!("Caught unused key: KeySIC"),
			Input::KeySLeft => info!("Caught unused key: KeySLeft"),
			Input::KeySMessage => info!("Caught unused key: KeySMessage"),
			Input::KeySMove => info!("Caught unused key: KeySMove"),
			Input::KeySNext => info!("Caught unused key: KeySNext"),
			Input::KeySOptions => info!("Caught unused key: KeySOptions"),
			Input::KeySPrevious => info!("Caught unused key: KeySPrevious"),
			Input::KeySPrint => info!("Caught unused key: KeySPrint"),
			Input::KeySRedo => info!("Caught unused key: KeySRedo"),
			Input::KeySReplace => info!("Caught unused key: KeySReplace"),
			Input::KeySRight => info!("Caught unused key: KeySRight"),
			Input::KeySResume => info!("Caught unused key: KeySResume"),
			Input::KeySSave => info!("Caught unused key: KeySSave"),
			Input::KeySSuspend => info!("Caught unused key: KeySSuspend"),
			Input::KeySUndo => info!("Caught unused key: KeySUndo"),
			Input::KeySuspend => info!("Caught unused key: KeySuspend"),
			Input::KeyUndo => info!("Caught unused key: KeyUndo"),
			Input::KeyResize => info!("Caught unused key: KeyResize"),
			Input::KeyEvent => info!("Caught unused key: KeyEvent"),
			Input::KeyMouse => info!("Caught unused key: KeyMouse"),
			Input::KeyA1 => info!("Caught unused key: KeyA1"),
			Input::KeyA3 => info!("Caught unused key: KeyA3"),
			Input::KeyB2 => info!("Caught unused key: KeyB2"),
			Input::KeyC1 => info!("Caught unused key: KeyC1"),
			Input::KeyC3 => info!("Caught unused key: KeyC3"),

			_ => info!("Caught unused key: Couldn't match key"),
		}
	}

	match board.move_autos(player1, player2) {
		Some(player) => board.update_score(player),
		_ => (),
	}
}
