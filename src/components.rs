pub struct Position {
	pub pos_x: usize,
	pub pos_y: usize,
}

impl Default for &Position {
	fn default() -> Self {
		&Position{ pos_x: 0, pos_y: 0 }
	}
}

pub struct Trajectory {
	pub trj_x: isize,
	pub trj_y: isize,
}

pub struct Score(pub u8);
