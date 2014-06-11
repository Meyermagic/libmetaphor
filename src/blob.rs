use object::{Object, Hasher};
use id::ID;

use std::io;
use std::io::{File, IoResult};



pub trait Blob {
	fn to_bytes(&self) -> IoResult<Vec<u8>>;
}

pub trait MutableBlob: Blob {
	fn from_bytes(&mut self, bytes: &[u8]) -> IoResult<()>;
}

/*
impl<B: Blob> Object for B {
	fn kind() -> &'static str { "blob" }

	fn hash<H: Hasher>(&self, hasher: &mut H) {
		hasher.input(self.to_bytes().as_slice());
	}
}
*/

//Implementations

/// A blob that wraps a filesystem path
pub struct FsBlob {
	path: Path
}

impl FsBlob {
	pub fn new(path: &Path) -> IoResult<FsBlob> {
		if !path.exists() { return Err(io::standard_error(io::PathDoesntExist)); }
		if !path.is_file() { return Err(io::standard_error(io::MismatchedFileTypeForOperation)); }
		//TODO: Do we want to force conversion to absolute?
		//let path = if path.is_absolute() { path.clone() } else { os::make_absolute(path) };
		return Ok(FsBlob{ path: path.clone() });
	}
}

impl Blob for FsBlob {
	fn to_bytes(&self) -> IoResult<Vec<u8>> {
		(try!(File::open(&self.path))).read_to_end()
	}
}

impl Object for FsBlob {
	fn kind(&self) -> &'static str { "blob" }

	fn hash<H: Hasher>(&self, hasher: &mut H) {
		hasher.input(self.to_bytes().unwrap().as_slice());
	}
}

impl MutableBlob for FsBlob {
	fn from_bytes(&mut self, bytes: &[u8]) -> IoResult<()> {
		(try!(File::create(&self.path))).write(bytes)
	}
}

pub struct EmptyBlob;

impl Blob for EmptyBlob {
	fn to_bytes(&self) -> IoResult<Vec<u8>> {
		Ok(vec!())
	}
}

impl Object for EmptyBlob {
	fn kind(&self) -> &'static str { "blob" }

	fn hash<H: Hasher>(&self, hasher: &mut H) {}
}


