use std::any;
use std::cell::{RefCell, Ref, RefMut};

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

		match self.borrow_component_vec::<ComponentType>() {
			Some(component_vec) => Some(component_vec.borrow()),
			None => None,
		}
	}

	pub fn borrow_component_vec_mut<ComponentType: 'static>(
		&self
	) -> Option<RefMut<Vec<Option<ComponentType>>>> {

		match self.borrow_component_vec::<ComponentType>() {
			Some(component_vec) => Some(component_vec.borrow_mut()),
			None => None,
		}
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
}
