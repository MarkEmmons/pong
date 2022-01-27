use console::Term;
use console::Key;

// Screen
const SCREEN_X: usize = 75;
const SCREEN_MID_X: usize = SCREEN_X / 2;
const SCREEN_Y: usize = 25;
const SCREEN_MID_Y: usize = SCREEN_Y / 2;

type Screen = [[char; SCREEN_X]; SCREEN_Y];

// ECS
struct Score(u8);
struct Position(usize, usize);

struct Board {
	score_components: Vec<Option<Score>>,
	position_components: Vec<Option<Position>>,
}

impl Board {

	fn new() -> Self {
		Self {
			score_components: Vec::new(),
			position_components: Vec::new(),
		}
	}

	fn new_entity(&mut self, score: Option<Score>, position: Option<Position>) {
		self.score_components.push(score);
		self.position_components.push(position);
	}
}

fn print_screen(screen: [[char; SCREEN_X]; SCREEN_Y]) {

	for row in screen.iter() {

		for pix in row.iter() {

			print!("{}", pix);
		}

		println!();
	}
}

fn main() {

	// Initialize Board
	let mut board = Board::new();

	// Player 1
	board.new_entity(
		Some(Score(0)),
		Some(Position(
			SCREEN_MID_Y,
			1,
		)));

	// Player 2
	board.new_entity(
		Some(Score(0)),
		Some(Position(
			SCREEN_MID_Y,
			SCREEN_X - 2,
		)));

	// Ball
	board.new_entity(
		None,
		Some(Position(
			SCREEN_MID_Y,
			SCREEN_MID_X,
		)));

	// Initialize Screen
	let screen: Screen = init_screen(&board);
    print_screen(screen);

	// Initialize Input
	let stdout = Term::buffered_stdout();

	loop {

		get_user_input(&stdout);
		update_screen();
		draw_screen();
	}
}

fn init_screen(board: &Board) -> Screen {

	let mut screen: Screen = [['*'; SCREEN_X]; SCREEN_Y];

	for r in 1..(screen.len() - 1) {

		for c in 1..(screen[r].len() - 1) {

			screen[r][c] = if c == SCREEN_MID_X {
				'|'
			} else {
				' '
			};
		}
	}

	let zip = board
		.score_components
		.iter()
		.zip(board.position_components.iter());

	let player_paddles =
		zip.filter_map(|(score, position): (&Option<Score>, &Option<Position>)| {
			Some((score.as_ref()?, position.as_ref()?))
		});

	for(score, position) in player_paddles {

		// Draw Paddle
		screen[position.0-1][position.1] = '|';
		screen[position.0][position.1] = '|';
		screen[position.0+1][position.1] = '|';

		// Draw Score
		let score_pos: usize = if position.1 < SCREEN_MID_X {
			SCREEN_MID_X / 2
		} else {
			(SCREEN_MID_X / 2) * 3
		};

		let scores: Vec<char> = score.0.to_string().chars().collect();
		screen[0][score_pos-1] = ' ';
		screen[0][score_pos] = scores[0];
		screen[0][score_pos+1] = ' ';
	}

	screen
}


fn get_user_input(stdout: &Term) {

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

fn update_screen() {}

fn draw_screen() {}
