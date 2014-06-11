use blob::{Blob, MutableBlob};
use diff::{Diff, Patch, Algorithm, DiskPatch};
use disk::{Disk, ToDisk, FromDisk};

use semver::Version;

use std::intrinsics;

use std::io::{IoResult, BufReader, MemWriter};
use libbsdiff;
use libbspatch;

pub trait Diff<P: Patch> {
	fn diff_blobs<T: Blob, U: Blob>(&self, blob_a: &T, blob_b: &U) -> IoResult<P>;
}

pub struct BsDiff;

impl Diff<BsPatch> for BsDiff {
	fn diff_blobs<T: Blob, U: Blob>(&self, blob_a: &T, blob_b: &U) -> IoResult<BsPatch> {
		let mut patch_writer = MemWriter::new();
		let old_bytes = try!(blob_a.to_bytes());
		let new_bytes = try!(blob_b.to_bytes());
		libbsdiff::diff(old_bytes.as_slice(), new_bytes.as_slice(), &mut patch_writer as &mut Writer);
		return Ok(BsPatch{new_size: new_bytes.len(), patch: patch_writer.unwrap()});
	}
}

pub struct BsPatch {
	new_size: u64,
	patch: Vec<u8>
}

impl Patch for BsPatch {
	fn patch_blob<T: MutableBlob>(&self, blob: &mut T) -> IoResult<()> {
		let mut new = Vec::from_elem(self.new_size, 0 as u8);
		let old = try!(blob.to_bytes());
		let mut reader = BufReader::new(self.patch.as_slice());
		libbspatch::patch(old.as_slice(), new.as_mut_slice(), &mut reader as &mut Reader);
		return blob.from_bytes(new);
	}
}

impl Algorithm for BsPatch {
	fn name() -> &'static str { "bspatch" }
	fn version() -> Version { Version{major: 0, minor: 1, patch: 1} }
}

impl ToDisk<DiskPatch, ()> for BsPatch {
	fn to_disk(&self) -> (DiskPatch, ()) {
		let body_writer = MemWriter::with_capacity(self.patch.len() + intrinsics::size_of::<u64>());
		body_writer.write_le_u64(self.new_size);
		body_writer.write(self.patch.as_slice());
		let body = body_writer.unwrap();
		return (DiskPatch{algorithm: self.name(), version: self.version(), body: body}, ());
	}
}

impl FromDisk<DiskPatch, ()> for BsPatch {
	fn from_disk(disk: &DiskPatch, stores: &mut ()) -> IoResult<BsPatch> {

	}
}

