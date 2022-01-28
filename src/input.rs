use console::{Key, Term};

pub fn get_user_input(stdout: &Term) {

	if let Ok(key) = stdout.read_key() {

		match key {

			Key::ArrowLeft => println!("Left"),
			Key::ArrowRight => println!("Right"),
			Key::ArrowUp => println!("Up"),
			Key::ArrowDown => println!("Down"),

			Key::Char(c) => {
				match c {
					'q' => todo!("quit"),
					_ => println!("{}", c),
				}
			}

			Key::Enter => println!("Enter"),
			Key::Escape => println!("Escape"),
			Key::Backspace => println!("Backspace"),
			Key::Home => println!("Home"),
			Key::End => println!("End"),
			Key::Tab => println!("Tab"),
			Key::BackTab => println!("BackTab"),
			Key::Alt => println!("Alt"),
			Key::Del => println!("Del"),
			Key::Shift => println!("Shift"),
			Key::Insert => println!("Insert"),
			Key::PageUp => println!("PageUp"),
			Key::PageDown => println!("PageDown"),

			_ => println!("Couldn't match key"),
		}
	}
}

