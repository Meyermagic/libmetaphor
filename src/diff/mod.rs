
use blob::{Blob, MutableBlob};
use disk::{Disk, ToDisk};
use object::{Object, Hasher};

use semver::Version;
use std::io::IoResult;

//pub use diff::bsdiff::{BsDiff, BsPatch};
//pub use diff::myersdiff::{MyersLineDiff, MyersLinePatch};
pub use diff::patience::{PatienceLineDiff, PatienceLinePatch};

//mod bsdiff;
//mod myersdiff;
mod patience;
mod common;

pub trait Diff<P: Patch> {
	fn diff_blobs<T: Blob, U: Blob>(&self, blob_a: &T, blob_b: &U) -> IoResult<P>;
}

pub trait Patch: ToDisk<DiskPatch, ()> {
	fn algorithm(_: Option<Self>) -> &'static str;
	fn patch_blob<'a, 'b, T: MutableBlob>(&'a self, blob: &'b mut T) -> IoResult<()>;
}

impl<'c, P: Patch> Patch for &'c P {
	fn algorithm(_: Option<&'c P>) -> &'static str {
		Patch::algorithm(None::<P>)
	}
	fn patch_blob<'c, 'b, T: MutableBlob>(&'c self, blob: &'b mut T) -> IoResult<()> {
		self.patch_blob(blob)
	}
}

impl<P: Patch> Patch for Box<P> {
	fn algorithm(_: Option<Box<P>>) -> &'static str {
		Patch::algorithm(None::<P>)
	}
	fn patch_blob<'c, 'b, T: MutableBlob>(&self, blob: &mut T) -> IoResult<()> {
		self.patch_blob(blob)
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
		unimplemented!();
	}
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		unimplemented!();
	}
}

impl ToDisk<DiskPatch, ()> for DiskPatch {
	#[inline]
	fn to_disk(&self) -> (DiskPatch, ()) {
		(self.clone(), ())
	}
}