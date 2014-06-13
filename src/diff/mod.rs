
use blob::{Blob, MutableBlob};
use disk::{Disk, ToDisk, FromDisk};
use database::Database;
use object::{Object, Hasher};

use semver::Version;
use std::io::IoResult;

use std::owned::AnyOwnExt;
use std::any::{Any, AnyRefExt};

//pub use diff::bsdiff::{BsDiff, BsPatch};
//pub use diff::myersdiff::{MyersLineDiff, MyersLinePatch};
pub use diff::patience::{PatienceLineDiff, PatienceLinePatch, DiskPatienceLinePatch};

use std::raw::TraitObject;
use std::intrinsics::TypeId;
use std::intrinsics;
use std::any::Any;
use std::mem::{transmute, transmute_copy};
use std::mem;

use std::fmt;
use std::fmt::{Show, Formatter};

//mod bsdiff;
//mod myersdiff;
mod patience;
mod common;



pub trait Diff<P: Patch> {
	fn diff_blobs<T: Blob, U: Blob>(&self, blob_a: &T, blob_b: &U) -> IoResult<P>;
}

pub trait Patch: ToDisk<DiskPatch, ()> {
	fn algorithm(&self) -> &'static str;
	fn patch_blob<'a, 'b, T: MutableBlob>(&'a self, blob: &'b mut T) -> IoResult<()>;
}

impl<'c, P: Patch> Patch for &'c P {
	fn algorithm(&self) -> &'static str {
		self.algorithm()
	}
	fn patch_blob<'c, 'b, T: MutableBlob>(&'c self, blob: &'b mut T) -> IoResult<()> {
		(*self).patch_blob(blob)
	}
}

impl<P: Patch> Patch for Box<P> {
	fn algorithm(&self) -> &'static str {
		self.algorithm()
	}
	fn patch_blob<'c, 'b, T: MutableBlob>(&self, blob: &mut T) -> IoResult<()> {
		(**self).patch_blob(blob)
	}
}

pub trait AnyPatch<'a> {
	fn as_ref<T>(&self) -> Option<&'a T>;
}


impl<'a> AnyPatch<'a> for Box<Patch> {

	#[inline]
	fn as_ref<T>(&self) -> Option<&'a T> {
		//if (*self).is::<T>() {
			unsafe {
				// Get the raw representation of the trait object
				let to: TraitObject = transmute_copy(self);

				// Extract the data pointer
				Some(transmute(to.data))
			}
		/*} else {
			None
		}*/
	}
}

/*
impl AnyOwnExt for Box<Patch> {
	#[inline]
	fn move<T: 'static>(self) -> Result<Box<T>, Box<Patch>> {
		if self.is::<T>() {
			unsafe {
				// Get the raw representation of the trait object
				let to: TraitObject =
				*mem::transmute::<&Box<Patch>, &TraitObject>(&self);

				// Prevent destructor on self being run
				intrinsics::forget(self);

				// Extract the data pointer
				Ok(mem::transmute(to.data))
			}
		} else {
			Err(self)
		}
	}
}

impl<'a> AnyRefExt<'a> for &'a Patch {
	#[inline]
	fn is<T: 'static>(self) -> bool {
		// Get TypeId of the type this function is instantiated with
		let t = TypeId::of::<T>();

		// Get TypeId of the type in the trait object
		let boxed = self.get_type_id();

		// Compare both TypeIds on equality
		t == boxed
	}

	#[inline]
	fn as_ref<T: 'static>(self) -> Option<&'a T> {
		if self.is::<T>() {
			unsafe {
				// Get the raw representation of the trait object
				let to: TraitObject = transmute_copy(&self);

				// Extract the data pointer
				Some(transmute(to.data))
			}
		} else {
			None
		}
	}
}
*/

impl Show for Box<Patch> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.pad("Box of fucking <Patch>")
	}
}

impl Patch for Box<Patch> {
	fn algorithm(&self) -> &'static str {
		self.algorithm()
	}
	fn patch_blob<'c, 'b, T: MutableBlob>(&self, blob: &mut T) -> IoResult<()> {
		let untrait = self.clone().as_ref::<PatienceLinePatch>();
		return untrait.unwrap().patch_blob(blob);
	}
}

impl ToDisk<DiskPatch, ()> for Box<Patch> {
	fn to_disk(&self) -> (DiskPatch, ()) {
		let untrait = self.clone().as_ref::<PatienceLinePatch>();
		return untrait.unwrap().to_disk();
	}
}

#[deriving(Clone)]
pub struct DiskPatch {
	algorithm: String,
	body: Vec<u8>
}

impl Object for DiskPatch {
	fn kind(&self) -> &'static str { "diskpatch" }
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		hasher.input(self.algorithm.as_bytes());
		//FIXME: Hash the version
		//hasher.input(self.version.)
		hasher.input(self.body.as_slice());
	}
}

impl Disk for DiskPatch {
	fn read<R: Reader>(reader: &mut R) -> IoResult<DiskPatch> {
		let algorithm: String = try!(Disk::read(reader));
		let byte_count = try!(reader.read_le_u64());
		let body: Vec<u8> = try!(reader.read_exact(byte_count as uint));
		return Ok(DiskPatch{algorithm: algorithm, body: body});
	}
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.algorithm.write(writer));
		try!(writer.write_le_u64(self.body.len() as u64));
		try!(writer.write(self.body.as_slice()));
		return Ok(());
	}
}

impl ToDisk<DiskPatch, ()> for DiskPatch {
	#[inline]
	fn to_disk(&self) -> (DiskPatch, ()) {
		(self.clone(), ())
	}
}

//FIXME: This should dynamically work for all algorithms
impl<D: Database> FromDisk<Box<Patch>, D> for DiskPatch {
	fn from_disk(&self, database: &mut D) -> IoResult<Box<Patch>> {
		match self.algorithm.as_slice() {
			"patience_line_patch" => {
				let sp_disk_patch = DiskPatienceLinePatch { patch: self };
				let patch = try!(sp_disk_patch.from_disk(database));
				return Ok(patch as Box<Patch>);
			},
			_ => {
				//FIXME: Error handling.
				unreachable!();
			}
		}
	}
}