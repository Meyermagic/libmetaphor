
use object::{Object, Hasher};
use id::ID;


pub trait Tag<'a> {
	fn key(&'a self) -> &'a str;
	fn value(&'a self) -> &'a str;
	fn targets(&'a self) -> &'a [ID];
}

/*
impl<T: Tag> Object for T {
	fn kind() -> &'static str { "tag" }
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		hasher.input(self.key().as_bytes());
		hasher.input(self.value().as_bytes());
	}
}
*/

pub struct DiskTag {
	key: String,
	value: String,
	targets: Vec<ID>
}

impl<'a> Tag<'a> for DiskTag {
	fn key(&'a self) -> &'a str { self.key.as_slice() }
	fn value(&'a self) -> &'a str { self.value.as_slice() }
	fn targets(&'a self) -> &'a [ID] { self.targets.as_slice() }
}

impl Object for DiskTag {
	fn kind(&self) -> &'static str { "tag" }
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		hasher.input(self.key().as_bytes());
		hasher.input(self.value().as_bytes());
	}
}
