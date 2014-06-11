
use object::Object;


use std::io::{IoResult, BufReader};

// Met-serializable objects
pub trait Disk {
	// Required methods
	//fn name() -> &'static str;
	//fn kind_size() -> (uint, Option<uint>);
	//fn name_size() -> (uint, Option<uint>);
	/// Deserialize from a reader
	fn read<R: Reader>(reader: &mut R) -> IoResult<Self>;
	/// Serialize into a writer
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
	/// Number of bytes to be written in write.
	//fn size(&self) -> uint;

	// Provided methods
	fn from_bytes<'a>(buf: &'a [u8]) -> IoResult<Self> {
		Disk::read(&mut BufReader::new(buf))
	}
}

//Maybe do this explicitly for tuples of D: Disk to some length, like how Haskell does long tuples
pub trait ToDisk<D: Disk, T> {
	fn to_disk(&self) -> (D, T);
}

impl<'a, D: Disk, T, TD: ToDisk<D, T>> ToDisk<D, T> for &'a TD {
	fn to_disk(&self) -> (D, T) {
		self.to_disk()
	}
}

impl<D: Disk, T, TD: ToDisk<D, T>> ToDisk<D, T> for Box<TD> {
	fn to_disk(&self) -> (D, T) {
		self.to_disk()
	}
}

//TODO: Should this be self or &self? &mut self if we end up needing mut to write?
//Hopefully this optimizes out but will allow simpler interfaces, only accepting ToDisk instead of 
//ToDisk and Disk with separate code paths
/*
impl<D: Disk> ToDisk<D, ()> for D {
	#[inline]
	fn to_disk(self) -> (D, ()) {
		(self, ())
	}
}
*/

//We might need the tuple thing 
pub trait FromDisk<D, S> {
	fn from_disk(disk: &D, stores: &mut S) -> IoResult<Self>;
}

//TODO: Should there be an implicit FromDisk for Disk types, line with ToDisk?
//impl<D: Disk> FromDisk<D, 