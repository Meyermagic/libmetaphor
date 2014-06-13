
use object::{Object, Hasher};
use id::{ID, TagID};

use disk::Disk;
use std::io::IoResult;

pub trait Tag<'a> {
	fn key(&'a self) -> &'a str;
	fn value(&'a self) -> &'a str;
	fn targets(&'a self) -> &'a [ID];
}


pub fn tag_id(key: &str, value: &str) -> TagID {
	let tag = DiskTag { key: String::from_str(key), value: String::from_str(value), targets: vec!()};
	return tag.id();
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

#[deriving(Clone)]
pub struct DiskTag {
	pub key: String,
	pub value: String,
	pub targets: Vec<ID>
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

impl Disk for DiskTag {
	fn read<R: Reader>(reader: &mut R) -> IoResult<DiskTag> {
		let key: String = try!(Disk::read(reader));
		let value: String = try!(Disk::read(reader));
		let targets: Vec<ID> = try!(Disk::read(reader));
		return Ok(DiskTag{key:key, value:value, targets:targets});
	}

	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.key.write(writer));
		try!(self.value.write(writer));
		try!(self.targets.write(writer));
		return Ok(());
	}
}
