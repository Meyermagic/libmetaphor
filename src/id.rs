use crypto::sha2::Sha256;
pub use std::cmp::{Ord, Ordering, Less, Greater, Equal};
use std::fmt::{Show, Formatter};
use std::fmt;
use std::io;
use serialize::{json, Decodable, Encodable, Encoder, Decoder};
use std::slice::MutableCloneableVector;

pub use crypto::digest::Digest;

use std::default::Default;

use serialize::hex::{ToHex, FromHex};

use disk::Disk;
use std::io::IoResult;

#[deriving(Hash)]
pub struct ID {
	pub id: [u8, ..32]
}

pub static EmptyID: ID = ID{id: [227, 176, 196, 66,  152, 252, 28,  20,
                                 154, 251, 244, 200, 153, 111, 185, 36,
                                 39,  174, 65,  228, 100, 155, 147, 76,
                                 164, 149, 153, 27,  120, 82,  184, 85]};

//FIXME: Should this be called ZeroID?
pub static NullID: ID = ID{id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]};


impl ID {
	pub fn from_hex(hex: &str) -> ID {
		let bytes = hex.from_hex().unwrap();
		let mut id = EmptyID;
		id.id.as_mut_slice().copy_from(bytes.as_slice());
		return id;
	}

	#[inline]
	pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
		self.id.as_slice()
	}
}

//FIXME: Should this return EmptyID or NullID?
impl Default for ID {
	fn default() -> ID { EmptyID }
}

impl Disk for ID {
	fn read<R: Reader>(reader: &mut R) -> IoResult<ID> {
		let mut id: ID = Default::default();
		let mut read = 0;

		// Read until a full ID has been read, or error.
		while read < id.id.len() {
			match reader.read(id.id.as_mut_slice()) {
				Ok(n) => read += n,
				Err(e) => { return Err(e); }
			}
		}
		return Ok(id);
	}

	#[inline]
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		writer.write(self.id.as_slice())
	}

	//TODO: Should this work like size_hint? I don't know if we ever don't efficiently know the size.
	//#[inline]
	//fn size(&self) -> uint { self.id.len() }
}

/*
impl<E, S: Encoder<E>> Encodable<S, E> for ID {
	fn encode(&self, s: &mut S) -> Result<(), E> {
		self.id.as_slice().encode(s)
	}
}

impl<E, D: Decoder<E>> Decodable<D, E> for ID {
	fn decode(d: &mut D) -> Result<ID, E> {
		let mut id = EmptyID;
		match Decodable::decode(d) {
			Ok(id_slice) => { let bytes: ~[u8] = id_slice; id.id.as_mut_slice().copy_from(bytes.as_slice()); Ok(id) },
			Err(err) => Err(err)
		}
	}
}*/

impl ToHex for ID {
	fn to_hex(&self) -> String {
		self.id.as_slice().to_hex()
	}
}


impl Clone for ID {
	#[inline]
	fn clone(&self) -> ID { *self }
}

impl Eq for ID {}

impl PartialEq for ID {
	#[inline]
	fn eq(&self, other: &ID) -> bool {
		self.id == other.id
	}
}

impl PartialOrd for ID {
	#[inline]
	fn lt(&self, other: &ID) -> bool {
		self.id.as_slice() < other.id.as_slice()
	}
}

impl Ord for ID {
	#[inline]
	fn cmp(&self, other: &ID) -> Ordering {
		self.id.as_slice().cmp(&other.id.as_slice())
	}
}

impl Show for ID {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.to_hex().fmt(f)
		//write!(f, r"{}", )
	}
}
