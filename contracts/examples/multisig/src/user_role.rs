derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone, Copy, PartialEq)]
pub enum UserRole {
	None,
	Proposer,
	BoardMember,
}

impl UserRole {
	pub fn can_propose(&self) -> bool {
		match *self {
			UserRole::BoardMember | UserRole::Proposer => true,
			UserRole::None => false,
		}
	}

	pub fn can_perform_action(&self) -> bool {
		self.can_propose()
	}

	pub fn can_sign(&self) -> bool {
		match *self {
			UserRole::BoardMember => true,
			_ => false,
		}
	}
}
