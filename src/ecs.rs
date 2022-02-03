use std::any;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use log::debug;
use log::{info, warn};

use crate::screen::{SCREEN_X, SCREEN_Y, SCREEN_MID_X, SCREEN_MID_Y};

#[derive(Debug)]
pub struct Position {
	pub pos_x: usize,
	pub pos_y: usize,
}

pub struct Trajectory {
	pub trj_x: isize,
	pub trj_y: isize,
}

pub struct Score(pub u8);

const INVALID_POSITION: Position = Position {
	pos_x: 0,
	pos_y: 0,
};

const PLAYER_1_ROW: usize = 1;
const PLAYER_2_ROW: usize = SCREEN_X - 2;

const PLAYER_1_X_DIRECTION: isize = -1;
const PLAYER_2_X_DIRECTION: isize = 1;

// Component
pub trait ComponentVec {
	fn as_any(&self) -> &dyn any::Any;
	fn as_any_mut(&mut self) -> &mut dyn any::Any;
	fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {

	fn as_any(&self) -> &dyn any::Any {
		self as &dyn any::Any
	}

	fn as_any_mut(&mut self) -> &mut dyn any::Any {
		self as &mut dyn any::Any
	}

	fn push_none(&mut self) {
		self.get_mut().push(None)
	}
}

// Board
pub struct Board {

	pub entities_count: usize,
	pub component_vecs: Vec<Box<dyn ComponentVec>>
}

impl Board {

	pub fn new() -> Self {
		Self {
			entities_count: 0,
			component_vecs: Vec::new(),
		}
	}

	pub fn new_entity(&mut self) -> usize {

		let entity_id = self.entities_count;
		for component_vec in self.component_vecs.iter_mut() {
			component_vec.push_none();
		}
		self.entities_count += 1;
		entity_id
	}

	pub fn add_component_to_entity<ComponentType: 'static>(
		&mut self,
		entity: usize,
		component: ComponentType,
	) {

		// Look for matching component type
		for component_vec in self.component_vecs.iter_mut() {
			if let Some(component_vec) = component_vec
				.as_any_mut()
				.downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
			{
				component_vec.get_mut()[entity] = Some(component);
				return;
			}
		}

		// No matching component storage exists yet
		let mut new_component_vec: Vec<Option<ComponentType>> =
			Vec::with_capacity(self.entities_count);

		// All existing entities don't have this component, add None
		for _ in 0..self.entities_count {
			new_component_vec.push(None);
		}

		// Give this Entity the Component
		new_component_vec[entity] = Some(component);
		self
			.component_vecs
			.push(
				Box::new(RefCell::new(new_component_vec))
			);
	}

	pub fn borrow_component_vec_ref<ComponentType: 'static>(
		&self
	) -> Option<Ref<Vec<Option<ComponentType>>>> {

		for component_vec in self.component_vecs.iter() {
			if let Some(component_vec) = component_vec
				.as_any()
				.downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
			{
				// Why does this work with return???
				return Some(component_vec.borrow());
			}
		}
	None
	}

	pub fn borrow_component_vec_mut<ComponentType: 'static>(
		&self
	) -> Option<RefMut<Vec<Option<ComponentType>>>> {

		for component_vec in self.component_vecs.iter() {
			if let Some(component_vec) = component_vec
				.as_any()
				.downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
			{
				// Why does this work with return???
				return Some(component_vec.borrow_mut());
			}
		}
	None
	}

	pub fn borrow_component_vec<ComponentType: 'static>(
		&self
	) -> Option<&RefCell<Vec<Option<ComponentType>>>> {

		for component_vec in self.component_vecs.iter() {
			if let Some(component_vec) = component_vec
				.as_any()
				.downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
			{
				// Why does this work with return???
				return Some(component_vec);
			}
		}
	None
	}

	pub fn move_entity(&mut self, entity: usize, direction: isize) {

		info!("Moving entity {}: {}", entity, direction);

		// Get current y position, will return 0 if position is None
		let cur_pos_y = self
			.borrow_component_vec_mut::<Position>()
			.unwrap()[entity]
			.as_ref()
			.unwrap_or(&INVALID_POSITION)
			.pos_y;

		// Invalid position -> unwrap returned NONE
		if cur_pos_y == 0 {
			info!("Couldn't find valid position for entity {}", entity);
			return;
		}

		let new_pos = match update_position(cur_pos_y, direction) {
			None => cur_pos_y,
			Some(new_pos_y) => new_pos_y,
		};

		// New position is out of bounds, don't update
		if new_pos <= 1
			|| new_pos >= SCREEN_Y-2 {
			info!("New position {} out of bounds", new_pos);
			return;
		}

		// Assign the new position value
		self
			.borrow_component_vec_mut::<Position>()
			.unwrap()[entity]
			.as_mut()
			.unwrap()
			.pos_y = new_pos;

		info!("Player position moved to {}", new_pos);
	}

	fn get_paddle_positions(&self, p_positions: Rc<&RefCell<Vec<Option<Position>>>>) -> Vec<Position> {

		let mut pos_vec: Vec<Position> = Vec::new();

		let p_positions = p_positions.borrow();

		let scores = self
			.borrow_component_vec_ref::<Score>().unwrap();

		let paddles = p_positions
			.iter()
			.zip(scores.iter())
			.filter_map(| (position, score) |
				Some((position.as_ref()?, score.as_ref()?))
			);

		for(position, _) in paddles {

			pos_vec.push(Position {
				pos_x: position.pos_x,
				pos_y: position.pos_y,
			});
		}

		pos_vec
	}

	pub fn move_autos(&mut self, player1: usize, player2: usize) -> Option<usize> {

		info!("Moving autos");

		let rc_positions = Rc::new(self
			.borrow_component_vec::<Position>()
			.unwrap()
		);

		let paddle_positions: Vec<Position> = self
			.get_paddle_positions(Rc::clone(&rc_positions));

		// Get the entities with trajectories
		let mut trajectories = self
			.borrow_component_vec_mut::<Trajectory>().unwrap();

		let mut positions = Rc::clone(&rc_positions).borrow_mut();
		let autos = positions
			.iter_mut()
			.zip(trajectories.iter_mut())
			.filter_map(| (position, trajectory) |
				Some((position.as_mut()?, trajectory.as_mut()?))
			);

		for(position, trajectory) in autos {

			// Get current position
			let cur_pos_x = position.pos_x;
			let cur_pos_y = position.pos_y;

			// Get current trajectory
			let cur_trj_x = trajectory.trj_x;
			let cur_trj_y = trajectory.trj_y;

			// Invalid positions
			if cur_pos_x == 0
				|| cur_pos_y == 0
				|| cur_pos_x > (SCREEN_X-2)
				|| cur_pos_y > (SCREEN_Y-2)
			{
				warn!("Couldn't find valid position for auto");
				return None;
			}

			// Calculate new position
			let new_pos_x =
				match update_position(cur_pos_x, cur_trj_x) {
					None => cur_pos_x,
					Some(new_pos_x) => new_pos_x,
				};

			let new_pos_y =
				match update_position(cur_pos_y, cur_trj_y) {
					None => cur_pos_y,
					Some(new_pos_y) => new_pos_y,
				};

			match new_pos_x {

				// Hit the left paddle or score
				PLAYER_1_ROW => {

					return detect_paddle_collisions(position,
						trajectory,
						&paddle_positions,
						PLAYER_1_ROW,
						PLAYER_1_X_DIRECTION,
						player2
					);
				}

				// Hit the right paddle or score
				PLAYER_2_ROW => {

					return detect_paddle_collisions(position,
						trajectory,
						&paddle_positions,
						PLAYER_2_ROW,
						PLAYER_2_X_DIRECTION,
						player1
					);
				}

				// Update position
				_ => {
					position.pos_x = new_pos_x;
					info!("Auto x position moved to {}", new_pos_x);
				}
			}

			if new_pos_y <= 0 {
				trajectory.trj_y = 1;
				info!("Auto y trajectory updated to 1");
			} else if new_pos_y >= SCREEN_Y-1 {
				trajectory.trj_y = -1;
				info!("Auto y trajectory updated to -1");
			} else {
				position.pos_y = new_pos_y;
				info!("Auto y position moved to {}", new_pos_y);
			}
		}
		None
	}

	pub fn update_score(&mut self, player: usize) {

		self.borrow_component_vec_mut::<Score>()
			.unwrap()[player]
			.as_mut()
			.unwrap()
			.0 += 1;
	}
}

fn detect_paddle_collisions(position: &mut Position,
	trajectory: &mut Trajectory,
	paddle_positions: &Vec<Position>,
	player_row: usize,
	direction: isize,
	scoring_player: usize
) -> Option<usize> {

	// Detect collisions
	for p in paddle_positions {

		if p.pos_x != player_row {
			continue;
		}

		let py = p.pos_y;

		// Top paddle hit
		if position.pos_y == py - 1 {

			position.pos_x = match update_position(
				position.pos_x,
				direction
			) {
				Some(new_pos_x) => new_pos_x,
				None => position.pos_x,
			};
			if position.pos_y > 2 {
				position.pos_y -= 1;
			}

			trajectory.trj_x = direction;
			trajectory.trj_y = -1;
			debug!("POW!!! Top paddle hit: {},{} {},{}",
				position.pos_x,
				position.pos_y,
				trajectory.trj_x,
				trajectory.trj_y,
			);

		// Middle paddle hit
		} else if position.pos_y == py {

			position.pos_x = match update_position(
				position.pos_x,
				direction
			) {
				Some(new_pos_x) => new_pos_x,
				None => position.pos_x,
			};

			trajectory.trj_x = direction;
			trajectory.trj_y = 0;
			debug!("POW!!! Middle paddle hit: {},{} {},{}",
				position.pos_x,
				position.pos_y,
				trajectory.trj_x,
				trajectory.trj_y,
			);

		// Bottom paddle hit
		} else if position.pos_y == py + 1 {

			position.pos_x = match update_position(
				position.pos_x,
				direction
			) {
				Some(new_pos_x) => new_pos_x,
				None => position.pos_x,
			};
			if position.pos_y < SCREEN_Y-3 {
				position.pos_y += 1;
			}

			trajectory.trj_x = direction;
			trajectory.trj_y = 1;
			debug!("POW!!! Bottom paddle hit: {},{} {},{}",
				position.pos_x,
				position.pos_y,
				trajectory.trj_x,
				trajectory.trj_y,
			);

		// Score!
		} else {

			position.pos_x = SCREEN_MID_X;
			position.pos_y = SCREEN_MID_Y;
			debug!("SCORE! {},{}",
				position.pos_x,
				position.pos_y,
			);

			return Some(scoring_player);
		}
	}
	None
}

// Add an isize to a usize and return a wrapped usize
fn update_position(pos: usize, dir: isize) -> Option<usize> {

	if dir.is_negative() {
		pos.checked_sub(dir.wrapping_abs() as usize)
	} else {
		pos.checked_add(dir as usize)
	}
}
