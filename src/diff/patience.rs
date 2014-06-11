
use std::collections::{HashMap, Bitv};
use std::cmp::min;
use std::hash::Hash;

use diff::common::{common_affix, lcs};
use diff::{Diff, Patch, DiskPatch};
use semver::Version;
use blob::{Blob, MutableBlob};
use disk::{Disk, ToDisk};
use std::io::IoResult;

#[deriving(Show,Decodable,Encodable)]
pub struct PatienceEdit<T> {
	old_index: uint,
	deleted: uint,
	inserted: Vec<T>,
}

#[deriving(Show)]
pub struct PatienceDelta {
	old_index: uint,
	new_index: uint,
	deleted: uint,
	inserted: uint,
}

#[deriving(Show,Decodable,Encodable)]
pub struct PatiencePatch<T> {
	edits: Vec<PatienceEdit<T>>,
}


pub struct PatienceDiff<'a, E> {
	old: &'a [E],
	new: &'a [E],
	old_diff: Bitv,
	new_diff: Bitv,
}


impl<'a, E: Eq+Clone+Hash> PatienceDiff<'a, E> {
	fn new(old: &'a [E], new: &'a [E]) -> PatienceDiff<'a, E> {
		PatienceDiff{
			old: old,
			new: new,
			old_diff: Bitv::new(old.len(), false),
			new_diff: Bitv::new(new.len(), false),
		}
	}

	fn diff(&mut self) {
		//FIXME: Do we every need to initialize a PatienceDiff without running diff() immediately afterwards?
		// Maybe to distribute them to other tasks or something? We could just send the references to the sequences

		//Count common prefix / suffix
		let (common_prefix, common_suffix) = common_affix(self.old, self.new);

		//Find unique lines by storing Some(line number) or None for non-unique lines
		let mut old_lines = HashMap::new();
		for i in range(common_prefix, self.old.len() - common_suffix) {
			old_lines.insert_or_update_with(self.old.get(i), Some(i), |k, v| { *v = None; });
		}

		let mut new_lines = HashMap::new();
		for i in range(common_prefix, self.new.len() - common_suffix) {
			new_lines.insert_or_update_with(self.new.get(i), Some(i), |k, v| { *v = None; });
		}

		//Pull out the unique lines from the previous map, storing just &element -> index
		let mut old_unique = HashMap::new();
		for (&k, &v) in old_lines.iter() {
			match v {
				Some(index) => { old_unique.insert(k, index); },
				None => {},
			}
		}

		let mut new_unique = HashMap::new();
		for (&k, &v) in new_lines.iter() {
			match v {
				Some(index) => { new_unique.insert(k, index); },
				None => {},
			}
		}
		//let mut old_unique = old_lines.iter().filter_map(|(k, v)| { if v == None { None } else { (k, v.unwrap()) } }).collect::<HashMap<_, _>>();
		//let mut new_unique = new_lines.iter().filter_map(|(k, v)| { if v == None { None } else { (k, v.unwrap()) } }).collect::<HashMap<_, _>>();

		//FIXME: Instead of pulling out indices, probably want to use sort_by and RawPtr::to_uint

		//Extract the indices of the common unique lines
		let mut old_common_indices = Vec::new();
		let mut new_common_indices = Vec::new();
		for (key, old_value) in old_unique.iter() {
			match new_unique.find(key) {
				Some(new_value) => { old_common_indices.push(*old_value); new_common_indices.push(*new_value); },
				None => {},
			}
		}

		//Sort the common unique line indices
		old_common_indices.sort();
		new_common_indices.sort();

		let mut old_common = Vec::with_capacity(old_common_indices.len());
		let mut new_common = Vec::with_capacity(new_common_indices.len());

		//Map the indices to pointers to the elements at those indices
		for &i in old_common_indices.iter() {
			old_common.push(self.old.get(i));
		}

		for &i in new_common_indices.iter() {
			new_common.push(self.new.get(i));
		}

		//Longest common subsequence of common unique lines
		let pairs = lcs(old_common.as_slice(), new_common.as_slice());

		//FIXME: Handle special cases? Done?
		if pairs.len() == 0 {
			//If there are no common pairs, lets just delete and insert the whole block, I guess.
			for i in range(common_prefix, self.old.len() - common_suffix) {
				self.old_diff.set(i, true);
			}
			for i in range(common_prefix, self.new.len() - common_suffix) {
				self.new_diff.set(i, true);
			}
			return;
		}

		//First, mark insertions / deletions between start of first common unique line and the last
		//prefix match that we calculated at the start.
		//FIXME: .clone here and other places, or dereference with * and implicit copy?
		let (old_match, new_match) = *pairs.get(0);
		let old_match_index = *old_common_indices.get(old_match);
		let new_match_index = *new_common_indices.get(new_match);
		
		let (pre_prefix, pre_suffix) = common_affix(self.old.slice(common_prefix,
			                                                         old_match_index),
		                                            self.new.slice(common_prefix,
		                                            	             new_match_index));
		//Note: We know pre_prefix must be 0, since we start at common_prefix
		//Mark deletions
		for i in range(common_prefix, old_match_index - pre_suffix) {
			self.old_diff.set(i, true);
		}
		//Mark insertions
		for i in range(common_prefix, new_match_index - pre_suffix) {
			self.new_diff.set(i, true);
		}

		//Next, mark insertions / deletions between each pair of common unique lines.
		let (mut prev_old_match, mut prev_new_match) = (old_match, new_match);
		let mut prev_old_match_index = old_match_index;
		let mut prev_new_match_index = new_match_index;
		for &(old_match, new_match) in pairs.iter().skip(1) {
			let old_match_index = *old_common_indices.get(old_match);
			let new_match_index = *new_common_indices.get(new_match);

			let (inner_prefix, inner_suffix) = common_affix(self.old.slice(prev_old_match_index + 1,
				                                                             old_match_index),
			                                                self.new.slice(prev_new_match_index + 1,
			                                                	             new_match_index));
			//Mark deletions
			for i in range((prev_old_match_index + 1) + inner_prefix, old_match_index - inner_suffix) {
				self.old_diff.set(i, true);
			}
			//Mark insertions
			for i in range((prev_new_match_index + 1) + inner_prefix, new_match_index - inner_suffix) {
				self.new_diff.set(i, true);
			}

			//Update previous
			prev_old_match = old_match;
			prev_new_match = new_match;
			prev_old_match_index = old_match_index;
			prev_new_match_index = new_match_index;
		}

		//Finally, mark insertions / deletions between final common unique line and first suffix match.
		let (post_prefix, post_suffix) = common_affix(self.old.slice(prev_old_match_index + 1,
			                                                           self.old.len() - common_suffix),
		                                              self.new.slice(prev_new_match_index + 1,
		                                              	             self.new.len() - common_suffix));
		//Note: we know post_suffix will be 0, since we end at len - common_suffix.
		//Mark deletions
		for i in range(prev_old_match_index + 1 + post_prefix, self.old.len() - common_suffix) {
			self.old_diff.set(i, true);
		}
		//Mark insertions
		for i in range(prev_new_match_index + 1 + post_prefix, self.new.len() - common_suffix) {
			self.new_diff.set(i, true);
		}
	}

	fn get_deltas(&self) -> Vec<PatienceDelta> {
		let mut deltas = vec!();

		let mut old_i = 0;
		let mut new_i = 0;

		while old_i < self.old.len() || new_i < self.new.len() {
			if old_i < self.old.len() && !self.old_diff.get(old_i) &&
			   new_i < self.new.len() && !self.new_diff.get(new_i) {
			  //Matches
				old_i += 1;
				new_i += 1;
			} else {
				let old_start = old_i;
				let new_start = new_i;

				//Deletions
				while old_i < self.old.len() && (new_i >= self.new.len() || self.old_diff.get(old_i)) {
					old_i += 1;
				}

				//Insertions
				while new_i < self.new.len() && (old_i >= self.old.len() || self.new_diff.get(new_i)) {
					new_i += 1;
				}

				//Assemble insertions + deletions into a delta
				if old_start < old_i || new_start < new_i {
					deltas.push(PatienceDelta{
						old_index: old_start,
						deleted: old_i - old_start,
						new_index: new_start,
						inserted: new_i - new_start,
					});
				}
			}
		}

		return deltas;
	}

	fn get_patch(&self) -> PatiencePatch<E> {
		PatiencePatch {
			edits: self.get_deltas().iter().map(|delta| {
				PatienceEdit {
					old_index: delta.old_index,
					deleted: delta.deleted,
					inserted: Vec::from_slice(self.new.slice(delta.new_index,
						                                       delta.new_index + delta.inserted)),
				}
			}).collect(),
		}
	}
}

#[deriving(Clone, Eq, PartialEq, Hash)]
enum LineBlob {
	OldBlob,
	NewBlob,
}
type LineNumber = uint;

#[deriving(Clone, Eq, PartialEq, Hash)]
struct LineReference(LineBlob, LineNumber);

#[deriving(Show,Decodable,Encodable)]
pub struct PatienceLinePatch(PatiencePatch<String>);
pub struct PatienceLineDiff;

//struct PatienceLineEdit(PatienceEdit<String>);
impl<'a> Diff<PatienceLinePatch> for PatienceLineDiff {
	fn diff_blobs<T: Blob, U: Blob>(&self, old_blob: &T, new_blob: &U) -> IoResult<PatienceLinePatch> {
		//FIXME: We shouldn't fail here on invalid UTF-8, doing so is pretty dumb.
		//Really, we could avoid working on strings, I suppose, but still split lines.
		//That'd handle malformed UTF-8 files

		//Create iterators over the lines in the old and new blobs
		let old_blob_bytes = try!(old_blob.to_bytes());
		let old_blob_str = String::from_utf8(old_blob_bytes).unwrap();
		let old_blob_lines: Vec<&str> = old_blob_str.as_slice().split('\n').collect();

		let new_blob_bytes = try!(new_blob.to_bytes());
		let new_blob_str = String::from_utf8(new_blob_bytes).unwrap();
		let new_blob_lines: Vec<&str> = new_blob_str.as_slice().split('\n').collect();

		//FIXME: We could also map to pointers to the first occurence of the line, and run the diff on the pointers
		//Map the lines to (LineVersion, uint) pairs. (where the uint is the line number in blob)
		let mut line_map = HashMap::new();

		let mut old_line_nums = vec!();
		for (n, line) in old_blob_lines.iter().enumerate() {
			old_line_nums.push(line_map.find_or_insert(line, LineReference(OldBlob, n)).clone());
		}

		let mut new_line_nums = vec!();
		for (n, line) in new_blob_lines.iter().enumerate() {
			new_line_nums.push(line_map.find_or_insert(line, LineReference(NewBlob, n)).clone());
		}

		//FIXME: If we need to optimize, we certainly don't need to make so many passes over every element (including those in the actual diff)

		//Run the diff on the mapped lines
		let mut differ = PatienceDiff::new(old_line_nums.as_slice(), new_line_nums.as_slice());
		differ.diff();
		let lineref_patch = differ.get_patch();
		return Ok(PatienceLinePatch(PatiencePatch{
			edits: lineref_patch.edits.iter().map(|edit| {
				PatienceEdit {
					old_index: edit.old_index,
					deleted: edit.deleted,
					inserted: edit.inserted.iter().map(|line_ref| {
						match line_ref {
							&LineReference(OldBlob, n) => String::from_str(*old_blob_lines.get(n)),
							&LineReference(NewBlob, n) => String::from_str(*new_blob_lines.get(n)),
						}
					}).collect(),
				}
			}).collect(),
		}));
	}
}

impl Patch for PatienceLinePatch {
	fn algorithm(_: Option<PatienceLinePatch>) -> &'static str { "patience_line_patch" }
	fn patch_blob<T: MutableBlob>(&self, blob: &mut T) -> IoResult<()> {
		let old_blob_bytes = try!(blob.to_bytes());
		let old_blob_str = String::from_utf8(old_blob_bytes).unwrap();
		let old_blob_lines: Vec<&str> = old_blob_str.as_slice().split('\n').collect();

		let mut new_blob_lines: Vec<String> = vec!();

		let mut i = 0;
		// Destructure the wrapper type to get the underlying patch
		let &PatienceLinePatch(ref inner_patch) = self;

		// Loop over the edits in the patch
		for edit in inner_patch.edits.iter() {
			// Add matched lines to output
			for j in range(i, edit.old_index) {
				new_blob_lines.push(String::from_str(*old_blob_lines.get(j)));
			}

			// Jump i ahead to the end of the matched lines
			i = edit.old_index;

			// Skip over the deleted lines
			i += edit.deleted;

			// Append the inserted lines
			new_blob_lines.push_all(edit.inserted.as_slice());
		}

		let new_blob_str = new_blob_lines.connect("\n");
		return blob.from_bytes(new_blob_str.as_bytes());
	}
}

impl ToDisk<DiskPatch, ()> for PatienceLinePatch {
	fn to_disk(&self) -> (DiskPatch, ()) {
		unimplemented!();
	}
}
