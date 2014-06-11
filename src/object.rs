use std::slice::MutableCloneableVector;
use crypto::sha2::Sha256;

use id::{ID, EmptyID, NullID};

pub use Hasher = crypto::digest::Digest;


pub trait Object {
	fn kind(&self) -> &'static str;
	//For tags and things where we might want to absuse IDs, should we have a separate hash_unique
	// that also verifies integrity? Something like that?
	//fn hash_unique<H: Hasher>(&self, hasher: &mut H);
	fn hash<H: Hasher>(&self, hasher: &mut H);
	
	fn inner_id(&self) -> ID {
		let mut hasher = Sha256::new();

		self.hash(&mut hasher);

		let mut object_id = EmptyID;
		hasher.result(object_id.id.as_mut_slice());
		return object_id;
	}

	fn id(&self) -> ID {
		let mut hasher = Sha256::new();

		hasher.input(self.kind().as_bytes());
		self.hash(&mut hasher);

		let mut object_id = EmptyID;
		hasher.result(object_id.id.as_mut_slice());
		return object_id;
	}

	//fn unique_id(&self) -> ID { self.id() }
}