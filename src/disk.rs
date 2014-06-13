
use object::Object;

use database::Database;
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

impl Disk for String {
	fn read<R: Reader>(reader: &mut R) -> IoResult<String> {
		let byte_count = try!(reader.read_le_u64());
		let bytes = try!(reader.read_exact(byte_count as uint));
		// FIXME: Error handling
		return Ok(String::from_utf8(bytes).unwrap());
	}

	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		let bytes = self.as_bytes();
		try!(writer.write_le_u64(bytes.len() as u64));
		try!(writer.write(bytes));
		return Ok(());
	}
}

impl Disk for Path {
	fn read<R: Reader>(reader: &mut R) -> IoResult<Path> {
		let byte_count = try!(reader.read_le_u64());
		let bytes = try!(reader.read_exact(byte_count as uint));
		// FIXME: Error handling
		return Ok(Path::new(bytes));
	}

	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		let bytes = self.as_vec();
		try!(writer.write_le_u64(bytes.len() as u64));
		try!(writer.write(bytes));
		return Ok(());
	}
}

/*
//Damn, this impl issue / lack of feature is annoying as hell.
impl Disk for Vec<u8> {
	fn read<R: Reader>(reader: &mut R) -> IoResult<Vec<u8>> {
		let entries = try!(reader.read_le_u64());
		return reader.read_exact(entries as uint);
	}

	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		let entries: u64 = self.len() as u64;
		try!(writer.write_le_u64(entries));
		return writer.write(self.as_slice());
	}
}
*/

impl<D: Disk> Disk for Vec<D> {
	fn read<R: Reader>(reader: &mut R) -> IoResult<Vec<D>> {
		let entries = try!(reader.read_le_u64());
		let mut result = Vec::with_capacity(entries as uint);
		for i in range(0, entries) {
			let entry: D = try!(Disk::read(reader));
			result.push(entry);
		}
		return Ok(result);
	}

	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		let entries: u64 = self.len() as u64;
		try!(writer.write_le_u64(entries));
		for v in self.iter() {
			try!(v.write(writer));
		}
		return Ok(());
	}
}

/*
pub type DiskArtifacts = (Vec<DiskTag>,
	                        Vec<DiskCommit>,
	                        Vec<DiskChangeSeq>,
	                        Vec<DiskChange>,
	                        Vec<DiskPatch>);
*/

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
		(**self).to_disk()
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
pub trait FromDisk<T, D: Database> {
	fn from_disk(&self, database: &mut D) -> IoResult<T>;
}
