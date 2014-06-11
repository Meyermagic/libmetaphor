use diff::{Diff, Patch, DiskPatch, Algorithm};

use collections::bitv::Bitv;
use blob::MutableBlob;

use disk::{Disk, FromDisk, ToDisk};

use std::io::IoResult;

struct MyersDelta<T> {
	old_i: uint,
	deleted: uint,
	inserted: Vec<T>
}

struct MyersPatch<T> {
	deltas: Vec<MyersDelta<T>>
}

struct MyersDiff<'a, T> {
	old: &'a [T],
	new: &'a [T],
	old_diff: Bitv,
	new_diff: Bitv
}

impl<'a, T: TotalEq> MyersDiff<'a, T> {
	fn new(old: &'a [T], new: &'a [T]) -> MyersDiff<'a, T> {
		MyersDiff{old: old, new: new,
		          old_diff: Bitv::new(old.len(), false),
		          new_diff: Bitv::new(new.len(), false)}
	}
}


//Public Implementations

//Myers diff of lines, w/ hashing of lines
//FIXME: Make sure the probability of collision is insignificant, since this is important.
// Also, benchmark against testing lines for equality directly, although I'd be surprised if there
//was no performance benefit to hashing first.
pub struct MyersLineDiff;

impl Diff<MyersLinePatch> for MyersLineDiff {
	fn diff_blobs<T: Blob, U: Blob>(&self, blob_a: &T, blob_b: &U) -> IoResult<MyersLinePatch> {
		let old_bytes = try!(blob_a.to_bytes());
		let new_bytes = try!(blob_b.to_bytes());

		// FIXME: Return error if files are not UTF-8 instead of failing
		let old_str = StrBuf::from_utf8(old_bytes);
		let new_str = StrBuf::from_utf8(new_bytes);

		let old_slice = old_str.as_slice();
		let new_slice = new_str.as_slice();

		let old_lines = old_slice.split('\n');
		let new_lines = new_slice.split('\n');

		let 

	}
}

pub struct MyersLinePatch {
	deltas: Vec<MyersDelta<~str>>
}

impl Patch for MyersLinePatch {
	fn patch_blob<T: MutableBlob>(&self, blob: &mut T) -> IoResult<()> {

	}
}

impl ToDisk<DiskPatch, ()> for MyersLinePatch {

}

impl FromDisk<DiskPatch, ()> for MyersLinePatch {
	fn from_disk(disk: &DiskPatch, stores: &mut ()) -> IoResult<MyersLinePatch> {

	}
}

