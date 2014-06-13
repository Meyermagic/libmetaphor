use id::{ID, CommitID, ChangeSeqID};
use object::{Object, Hasher};

use disk::{Disk, ToDisk, FromDisk};

use std::io::IoResult;
use change::RecChange;
use diff::Patch;

use change::{DiskChangeSeq, DiskChange};
use diff::DiskPatch;

#[deriving(Show,Clone)]
pub enum CommitType {
	Basic(CommitID),
	Merge(CommitID, CommitID)
}

impl Disk for CommitType {
	fn read<R: Reader>(reader: &mut R) -> IoResult<CommitType> {
		let variant = try!(reader.read_le_u64());
		// FIXME: Is it more idiomatic to use the match as an expression?
		match variant {
			1 => {
				let cid: CommitID = try!(Disk::read(reader));
				return Ok(Basic(cid));
			},
			2 => {
				let cid1: CommitID = try!(Disk::read(reader));
				let cid2: CommitID = try!(Disk::read(reader));
				return Ok(Merge(cid1, cid2));
			},
			_ => { unreachable!(); }
		}
		unreachable!();
	}

	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		match self {
			&Basic(cid) => {
				try!(writer.write_le_u64(1));
				try!(cid.write(writer));
			},
			&Merge(cid1, cid2) => {
				try!(writer.write_le_u64(2));
				try!(cid1.write(writer));
				try!(cid2.write(writer));
			},
		}
		return Ok(());
	}
}

pub struct MemCommit<P> {
	pub author: String,
	pub short: String,
	pub long: String,
	pub family: CommitType,
	pub changes: Vec<RecChange<P>>,
}

impl<P: Patch> MemCommit<P> {
	pub fn new(author: &str, short: &str, long: &str, parent: CommitID, changes: Vec<RecChange<P>>) -> MemCommit<P> {
		MemCommit {
			author: String::from_str(author),
			short: String::from_str(short),
			long: String::from_str(long),
			family: Basic(parent),
			changes: changes,
		}
	}
}

impl<P: Patch> ToDisk<DiskCommit, (Vec<DiskChangeSeq>, Vec<DiskChange>, Vec<DiskPatch>)> for MemCommit<P> {
	fn to_disk(&self) -> (DiskCommit, (Vec<DiskChangeSeq>, Vec<DiskChange>, Vec<DiskPatch>)) {
		let (diskchangeseq, (other_changeseqs, other_changes, other_patches)) = self.changes.to_disk();
		let csid = diskchangeseq.id();
		let other_changeseqs = other_changeseqs.append_one(diskchangeseq);
		return (DiskCommit{
			author: self.author.clone(),
			short: self.short.clone(),
			long: self.long.clone(),
			family: self.family,
			changes: csid,
		}, (other_changeseqs, other_changes, other_patches));
	}
}


#[deriving(Show,Clone)]
pub struct DiskCommit {
	pub author: String,
	pub short: String,
	pub long: String,
	pub family: CommitType,
	pub changes: ChangeSeqID,
}

/*
pub impl DiskCommit {
	
}
*/

impl Disk for DiskCommit {
	fn read<R: Reader>(reader: &mut R) -> IoResult<DiskCommit> {
		let author: String = try!(Disk::read(reader));
		let short: String = try!(Disk::read(reader));
		let long: String = try!(Disk::read(reader));
		let family: CommitType = try!(Disk::read(reader));
		let changes: ChangeSeqID = try!(Disk::read(reader));
		return Ok(DiskCommit{author: author,
		                     short: short,
		                     long: long,
		                     family: family,
		                     changes: changes});
	}

	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.author.write(writer));
		try!(self.short.write(writer));
		try!(self.long.write(writer));
		try!(self.family.write(writer));
		try!(self.changes.write(writer));
		return Ok(());
	}
}

impl Object for DiskCommit {
	fn kind(&self) -> &'static str { "commit" }
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		hasher.input(self.author.as_bytes());
		hasher.input(self.short.as_bytes());
		hasher.input(self.long.as_bytes());
		match self.family {
			Basic(cid) => {
				hasher.input("basic".as_bytes());
				hasher.input(cid.as_bytes());
			},
			Merge(cid1, cid2) => {
				hasher.input("merge".as_bytes());
				hasher.input(cid1.as_bytes());
				hasher.input(cid2.as_bytes());
			}
		}
		hasher.input(self.changes.as_bytes());
	}
}