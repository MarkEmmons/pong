use std::any;
use std::cell::{RefCell, RefMut};

use log::{info, warn};

use crate::screen::{SCREEN_X, SCREEN_Y}; //, SCREEN_MID_X, SCREEN_MID_Y};

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

	pub fn move_player(&mut self, player: usize, direction: isize) {

		info!("Moving player {}: {}", player, direction);

		// Get current y position, will return 0 if position is None
		let cur_pos_y = self
			.borrow_component_vec_mut::<Position>()
			.unwrap()[player]
			.as_ref()
			.unwrap_or(&INVALID_POSITION)
			.pos_y;

		// Invalid position -> unwrap returned NONE
		if cur_pos_y == 0 {
			info!("Couldn't find valid position for player {}", player);
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
			.unwrap()[player]
			.as_mut()
			.unwrap()
			.pos_y = new_pos;

		info!("Player position moved to {}", new_pos);
	}

	pub fn move_balls(&mut self) {

		info!("Moving balls");

		// Get the ball entities
		let mut positions = self
			.borrow_component_vec_mut::<Position>().unwrap();

		let mut trajectories = self
			.borrow_component_vec_mut::<Trajectory>().unwrap();

		let zip = positions.iter_mut().zip(trajectories.iter_mut());
		let iter = zip.filter_map(| (position, trajectory) | Some((position.as_mut()?, trajectory.as_mut()?)));

		for(position, trajectory) in iter {

			// Get current position
			let cur_pos_x = position.pos_x;
			let cur_pos_y = position.pos_y;

			// Get current trajectory
			let cur_trj_x = trajectory.trj_x;
			let cur_trj_y = trajectory.trj_y;

			// Invalid position
			if cur_pos_x == 0 || cur_pos_y == 0 {
				warn!("Couldn't find valid position for ball");
				return;
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

			// Calculate scores / paddle collisions
			match new_pos_x {

				// Hit the left paddle or score
				PLAYER_1_ROW => {

					// Check for paddle collision
					//let mut paddle_positions: Vec<(usize, usize)> = Vec::new();
					//for paddle_position in paddle_positions {

					//	let px = paddle_position.0;
					//	let py = paddle_position.1;

					//	if px != PLAYER_1_ROW {
					//		continue;
					//	}

					//	// Paddle-Top
					//	if new_pos_y == py + 1 { trajectory.trj_x = 1; trajectory.trj_y -= 1; }
					//	// Paddle-Middle
					//	else if new_pos_y == py { trajectory.trj_x = 1; trajectory.trj_y -= 1; }
					//	// Paddle-Bottom
					//	else if new_pos_y == py - 1 { trajectory.trj_x = 1; trajectory.trj_y -= 1; }
					//	// Score!
					//	else {
					//		position.pos_x = SCREEN_MID_X;
					//		position.pos_y = SCREEN_MID_Y;
					//	}

					//}

					// TODO
					trajectory.trj_x = 1;
					info!("Ball x trajectory updated to 1");
				}

				// Hit the right paddle or score
				PLAYER_2_ROW => {
					trajectory.trj_x = -1;
					info!("Ball x trajectory updated to -1");
				}

				// Update position
				_ => {
					position.pos_x = new_pos_x;
					info!("Ball x position moved to {}", new_pos_x);
				}
			}

			if new_pos_y <= 0 {
				trajectory.trj_y = 1;
				info!("Ball y trajectory updated to 1");
			} else if new_pos_y >= SCREEN_Y-1 {
				trajectory.trj_y = -1;
				info!("Ball y trajectory updated to -1");
			} else {
				position.pos_y = new_pos_y;
				info!("Ball y position moved to {}", new_pos_y);
			}
		}
	}
}

// Add an isize to a usize and return a wrapped usize
#[allow(dead_code)]
fn update_position(pos: usize, dir: isize) -> Option<usize> {

	if dir.is_negative() {
		pos.checked_sub(dir.wrapping_abs() as usize)
	} else {
		pos.checked_add(dir as usize)
	}
}
