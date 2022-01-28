pub struct Score(pub u8);
pub struct Position(pub usize, pub usize);

pub struct Board {
	pub score_components: Vec<Option<Score>>,
	pub position_components: Vec<Option<Position>>,
}

impl Board {

	pub fn new() -> Self {
		Self {
			score_components: Vec::new(),
			position_components: Vec::new(),
		}
	}

	pub fn new_entity(&mut self, score: Option<Score>, position: Option<Position>) {
		self.score_components.push(score);
		self.position_components.push(position);
	}
}
