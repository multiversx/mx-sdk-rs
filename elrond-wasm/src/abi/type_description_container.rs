use super::*;
use alloc::string::String;
use hashbrown::HashMap;

pub trait TypeDescriptionContainer {
	fn new() -> Self;

	fn contains_type(&self, type_name: &str) -> bool;

	fn insert(&mut self, type_name: String, type_description: TypeDescription);

	fn insert_all(&mut self, other: &Self);
}

#[derive(Debug)]
pub struct TypeDescriptionContainerImpl(pub HashMap<String, TypeDescription>);

impl TypeDescriptionContainer for TypeDescriptionContainerImpl {
	fn new() -> Self {
		TypeDescriptionContainerImpl(HashMap::new())
	}

	fn contains_type(&self, type_name: &str) -> bool {
		self.0.contains_key(type_name)
	}

	fn insert(&mut self, type_name: String, type_description: TypeDescription) {
		self.0.insert(type_name, type_description);
	}

	fn insert_all(&mut self, other: &Self) {
		for (key, value) in other.0.iter() {
			self.0.insert(key.clone(), value.clone());
		}
	}
}
