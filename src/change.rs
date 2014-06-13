use object::{Object, Hasher};
use tree::{Tree, MutableTree};
use blob::{Blob, MutableBlob};
use id::{ID, EmptyID, NullID, PatchID, ChangeID, ChangeSeqID};
use diff::{DiskPatch, Diff, Patch};

use disk::{Disk, ToDisk, FromDisk};


use database::Database;
use std::fmt;

use std::io::IoResult;


pub trait Change {
	fn patch_tree<B: MutableBlob, T: MutableTree<B>>(&self, tree: &mut T) -> IoResult<()>;
}


pub enum RecChange<P> {
	CreateTree(Path),
	DeleteTree(Path),
	MoveTree(Path, Path),
	CreateBlob(Path),
	DeleteBlob(Path),
	MoveBlob(Path, Path),
	ModifyBlob(Path, P),
	ApplyChanges(Path, Vec<RecChange<P>>)
}



impl<P: Patch> fmt::Show for RecChange<P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&CreateTree(ref path) => write!(f, "CreateTree({})", path.display()),
			&DeleteTree(ref path) => write!(f, "DeleteTree({})", path.display()),
			&MoveTree(ref path_a, ref path_b) => write!(f, "MoveTree({}, {})", path_a.display(), path_b.display()),
			&CreateBlob(ref path) => write!(f, "CreateBlob({})", path.display()),
			&DeleteBlob(ref path) => write!(f, "DeleteBlob({})", path.display()),
			&MoveBlob(ref path_a, ref path_b) => write!(f, "MoveBlob({}, {})", path_a.display(), path_b.display()),
			&ModifyBlob(ref path, _) => write!(f, "ModifyBlob({}, [patch])", path.display()),
			&ApplyChanges(ref path, ref change_seq) => write!(f, "ApplyChanges({}, {})", path.display(), change_seq),
		}
	}
}

impl<P: Patch> Change for Vec<RecChange<P>> {
	fn patch_tree<B: MutableBlob, T: MutableTree<B>>(&self, tree: &mut T) -> IoResult<()> {
		for change in self.iter() {
			//FIXME: We shouldn't allow half-completed patches
			try!(change.patch_tree(tree));
		}
		return Ok(());
	}
}

impl<P: Patch> Change for RecChange<P> {
	fn patch_tree<B: MutableBlob, T: MutableTree<B>>(&self, tree: &mut T) -> IoResult<()> {
		match self {
			&CreateTree(ref path) => tree.create_tree(path),
			&DeleteTree(ref path) => tree.delete_tree(path),
			&MoveTree(ref path_a, ref path_b) => tree.move_tree(path_a, path_b),
			&CreateBlob(ref path) => tree.create_blob(path),
			&DeleteBlob(ref path) => tree.delete_blob(path),
			&MoveBlob(ref path_a, ref path_b) => tree.move_blob(path_a, path_b),
			&ModifyBlob(ref path, ref patch) => tree.modify_blob(path, patch),
			&ApplyChanges(ref path, ref change_seq) => tree.modify_tree(path, change_seq),
		}
	}
}

impl<P: Patch> ToDisk<DiskChangeSeq, (Vec<DiskChangeSeq>, Vec<DiskChange>, Vec<DiskPatch>)> for Vec<RecChange<P>> {
	fn to_disk(&self) -> (DiskChangeSeq, (Vec<DiskChangeSeq>, Vec<DiskChange>, Vec<DiskPatch>)) {
		let mut inner_changeseqs: Vec<DiskChangeSeq> = vec!();
		let mut inner_changes: Vec<DiskChange> = vec!();
		let mut inner_patches: Vec<DiskPatch> = vec!();
		let mut change_ids = Vec::with_capacity(self.len());
		for change in self.iter() {
			let (diskchange, (other_changeseqs, other_changes, other_patches)) = change.to_disk();
			change_ids.push(diskchange.id());
			inner_changeseqs.push_all(other_changeseqs.as_slice());
			inner_changes.push_all(other_changes.as_slice());
			inner_patches.push_all(other_patches.as_slice());
			inner_changes.push(diskchange);
		}
		return (VecChangeSeq{changes: change_ids}, (inner_changeseqs, inner_changes, inner_patches));
	}
}

impl<P: Patch> ToDisk<DiskChange, (Vec<DiskChangeSeq>, Vec<DiskChange>, Vec<DiskPatch>)> for RecChange<P> {
	fn to_disk(&self) -> (DiskChange, (Vec<DiskChangeSeq>, Vec<DiskChange>, Vec<DiskPatch>)) {
		match self {
			&CreateTree(ref path) => { return (CreateTreeF(path.clone()), (vec!(), vec!(), vec!())); },
			&DeleteTree(ref path) => { return (DeleteTreeF(path.clone()), (vec!(), vec!(), vec!())); },
			&MoveTree(ref path_a, ref path_b) => { return (MoveTreeF(path_a.clone(), path_b.clone()), (vec!(), vec!(), vec!())); },
			&CreateBlob(ref path) => { return (CreateBlobF(path.clone()), (vec!(), vec!(), vec!())); },
			&DeleteBlob(ref path) => { return (DeleteBlobF(path.clone()), (vec!(), vec!(), vec!())); },
			&MoveBlob(ref path_a, ref path_b) => { return (MoveBlobF(path_a.clone(), path_b.clone()), (vec!(), vec!(), vec!())); },
			&ModifyBlob(ref path, ref patch) => {
				let (disk_patch, _) = patch.to_disk();
				return (ModifyBlobF(path.clone(), disk_patch.id()), (vec!(), vec!(), vec!(disk_patch)));
			},
			&ApplyChanges(ref path, ref change_seq) => {
				let (disk_changeseq, (other_changeseqs, other_changes, other_patches)) = change_seq.to_disk();
				let disk_change = ApplyChangesF(path.clone(), disk_changeseq.id());
				let other_changeseqs = other_changeseqs.append_one(disk_changeseq);
				return (disk_change, (other_changeseqs, other_changes, other_patches));
			}
		}
	}
}


#[deriving(Clone)]
pub enum FlatChange<P, S> {
	CreateTreeF(Path),
	DeleteTreeF(Path),
	MoveTreeF(Path, Path),
	CreateBlobF(Path),
	DeleteBlobF(Path),
	MoveBlobF(Path, Path),
	ModifyBlobF(Path, P),
	ApplyChangesF(Path, S)
}

impl<P: Patch, S: Change> Change for FlatChange<P, S> {
	fn patch_tree<B: MutableBlob, T: MutableTree<B>>(&self, tree: &mut T) -> IoResult<()> {
		match self {
			&CreateTreeF(ref path) => tree.create_tree(path),
			&DeleteTreeF(ref path) => tree.delete_tree(path),
			&MoveTreeF(ref path_a, ref path_b) => tree.move_tree(path_a, path_b),
			&CreateBlobF(ref path) => tree.create_blob(path),
			&DeleteBlobF(ref path) => tree.delete_blob(path),
			&MoveBlobF(ref path_a, ref path_b) => tree.move_blob(path_a, path_b),
			&ModifyBlobF(ref path, ref patch) => tree.modify_blob(path, patch),
			&ApplyChangesF(ref path, ref change_seq) => tree.modify_tree(path, change_seq)
		}
	}
}

impl<P: Object, S: Object> Object for FlatChange<P, S> {
	fn kind(&self) -> &'static str {
		"change"
	}

	fn hash<H: Hasher>(&self, hasher: &mut H) {
		match self {
			//FIXME: Probably want to use a hash!(bytes, bytes, bytes) macro or a function here.
			// Rust is expression-oriented, though. I could just return a ~[&[u8]] or something and have
			// hasher.input once at the end.
			&CreateTreeF(ref path) => {
				hasher.input("create_tree".as_bytes());
				hasher.input(path.as_vec());
			},
			&DeleteTreeF(ref path) => {
				hasher.input("delete_tree".as_bytes());
				hasher.input(path.as_vec());
			},
			&MoveTreeF(ref path_a, ref path_b) => {
				hasher.input("move_tree".as_bytes());
				hasher.input(path_a.as_vec());
				hasher.input(path_b.as_vec());
			},
			&CreateBlobF(ref path) => {
				hasher.input("create_blob".as_bytes());
				hasher.input(path.as_vec());
			},
			&DeleteBlobF(ref path) => {
				hasher.input("delete_blob".as_bytes());
				hasher.input(path.as_vec());
			},
			&MoveBlobF(ref path_a, ref path_b) => {
				hasher.input("move_blob".as_bytes());
				hasher.input(path_a.as_vec());
				hasher.input(path_b.as_vec());
			},
			&ModifyBlobF(ref path, ref patch) => {
				hasher.input("modify_blob".as_bytes());
				hasher.input(path.as_vec());
				hasher.input(patch.id().as_bytes());
			},
			&ApplyChangesF(ref path, ref change_seq) => {
				hasher.input("apply_changes".as_bytes());
				hasher.input(path.as_vec());
				hasher.input(change_seq.id().as_bytes());
			}
		}
	}
}

pub type DiskChange = FlatChange<PatchID, ChangeSeqID>;

#[deriving(Clone)]
pub struct VecChangeSeq<T> {
	changes: Vec<T>
}

pub type DiskChangeSeq = VecChangeSeq<ChangeID>;

/*
//I wish we had overlapping/non-overlapping impls
impl Object for DiskChange {
	fn kind(&self) -> &'static str {
		"change"
	}

	fn hash<H: Hasher>(&self, hasher: &mut H) {
		match self {
			//FIXME: Probably want to use a hash!(bytes, bytes, bytes) macro or a function here.
			// Rust is expression-oriented, though. I could just return a ~[&[u8]] or something and have
			// hasher.input once at the end.
			&CreateTreeF(ref path) => {
				hasher.input("create_tree".as_bytes());
				hasher.input(path.as_vec());
			},
			&DeleteTreeF(ref path) => {
				hasher.input("delete_tree".as_bytes());
				hasher.input(path.as_vec());
			},
			&MoveTreeF(ref path_a, ref path_b) => {
				hasher.input("move_tree".as_bytes());
				hasher.input(path_a.as_vec());
				hasher.input(path_b.as_vec());
			},
			&CreateBlobF(ref path) => {
				hasher.input("create_blob".as_bytes());
				hasher.input(path.as_vec());
			},
			&DeleteBlobF(ref path) => {
				hasher.input("delete_blob".as_bytes());
				hasher.input(path.as_vec());
			},
			&MoveBlobF(ref path_a, ref path_b) => {
				hasher.input("move_blob".as_bytes());
				hasher.input(path_a.as_vec());
				hasher.input(path_b.as_vec());
			},
			&ModifyBlobF(ref path, ref patch) => {
				hasher.input("modify_blob".as_bytes());
				hasher.input(path.as_vec());
				hasher.input(patch.as_bytes());
			},
			&ApplyChangesF(ref path, ref change_seq) => {
				hasher.input("apply_changes".as_bytes());
				hasher.input(path.as_vec());
				hasher.input(change_seq.as_bytes());
			}
		}
	}
}
*/


impl Object for DiskChangeSeq {
	fn kind(&self) -> &'static str { "changeseq" }

	fn hash<H: Hasher>(&self, hasher: &mut H) {
		for v in self.changes.iter() {
			hasher.input(v.as_bytes());
		}
	}
}

impl Disk for DiskChangeSeq {
	fn read<R: Reader>(reader: &mut R) -> IoResult<DiskChangeSeq> {
		let inner: Vec<ChangeID> = try!(Disk::read(reader));
		return Ok(VecChangeSeq{changes: inner});
	}
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.changes.write(writer));
		return Ok(());
	}
}


impl Disk for DiskChange {
	fn read<R: Reader>(reader: &mut R) -> IoResult<DiskChange> {
		let variant = try!(reader.read_le_u64());
		match variant {
			1 => {
				let path: Path = try!(Disk::read(reader));
				return Ok(CreateTreeF(path));
			},
			2 => {
				let path: Path = try!(Disk::read(reader));
				return Ok(DeleteTreeF(path));
			},
			3 => {
				let path_a: Path = try!(Disk::read(reader));
				let path_b: Path = try!(Disk::read(reader));
				return Ok(MoveTreeF(path_a, path_b));
			},
			4 => {
				let path: Path = try!(Disk::read(reader));
				return Ok(CreateBlobF(path));
			},
			5 => {
				let path: Path = try!(Disk::read(reader));
				return Ok(DeleteBlobF(path));
			},
			6 => {
				let path_a: Path = try!(Disk::read(reader));
				let path_b: Path = try!(Disk::read(reader));
				return Ok(MoveTreeF(path_a, path_b));
			},
			7 => {
				let path: Path = try!(Disk::read(reader));
				let patch: PatchID = try!(Disk::read(reader));
				return Ok(ModifyBlobF(path, patch));
			},
			8 => {
				let path: Path = try!(Disk::read(reader));
				let change_seq: ChangeSeqID = try!(Disk::read(reader));
				return Ok(ApplyChangesF(path, change_seq));
			},
			_ => { unreachable!(); }
		}
	}
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		match self {
			&CreateTreeF(ref path) => {
				try!(writer.write_le_u64(1));
				try!(path.write(writer));
			},
			&DeleteTreeF(ref path) => {
				try!(writer.write_le_u64(2));
				try!(path.write(writer));
			},
			&MoveTreeF(ref path_a, ref path_b) => {
				try!(writer.write_le_u64(3));
				try!(path_a.write(writer));
				try!(path_b.write(writer));
			},
			&CreateBlobF(ref path) => {
				try!(writer.write_le_u64(4));
				try!(path.write(writer));
			},
			&DeleteBlobF(ref path) => {
				try!(writer.write_le_u64(5));
				try!(path.write(writer));
			},
			&MoveBlobF(ref path_a, ref path_b) => {
				try!(writer.write_le_u64(6));
				try!(path_a.write(writer));
				try!(path_b.write(writer));
			},
			&ModifyBlobF(ref path, ref patch) => {
				try!(writer.write_le_u64(7));
				try!(path.write(writer));
				try!(patch.write(writer));
			},
			&ApplyChangesF(ref path, ref change_seq) => {
				try!(writer.write_le_u64(8));
				try!(path.write(writer));
				try!(change_seq.write(writer));
			}
		}
		return Ok(());
	}
}

impl<D: Database> FromDisk<RecChange<Box<Patch>>, D> for DiskChange {
	fn from_disk(&self, database: &mut D) -> IoResult<RecChange<Box<Patch>>> {
		match self {
			&CreateTreeF(ref path) => {
				return Ok(CreateTree(path.clone()));
			},
			&DeleteTreeF(ref path) => {
				return Ok(DeleteTree(path.clone()));
			},
			&MoveTreeF(ref path_a, ref path_b) => {
				return Ok(MoveTree(path_a.clone(), path_b.clone()));
			},
			&CreateBlobF(ref path) => {
				return Ok(CreateBlob(path.clone()));
			},
			&DeleteBlobF(ref path) => {
				return Ok(DeleteBlob(path.clone()));
			},
			&MoveBlobF(ref path_a, ref path_b) => {
				return Ok(MoveBlob(path_a.clone(), path_b.clone()));
			},
			&ModifyBlobF(ref path, ref patch) => {
				//FIXME: Error handling.
				let disk_patch = database.get_patch(patch.clone()).unwrap();
				let box_patch: Box<Patch> = try!(disk_patch.from_disk(database));
				return Ok(ModifyBlob(path.clone(), box_patch));
			},
			&ApplyChangesF(ref path, ref change_seq) => {
				//FIXME: Error handling
				let disk_changeseq = database.get_changeseq(change_seq.clone()).unwrap();
				let real_changeseq = try!(disk_changeseq.from_disk(database));
				return Ok(ApplyChanges(path.clone(), real_changeseq));
			}
		}
	}
}


impl<D: Database> FromDisk<Vec<RecChange<Box<Patch>>>, D> for DiskChangeSeq {
	fn from_disk(&self, database: &mut D) -> IoResult<Vec<RecChange<Box<Patch>>>> {
		let mut change_seq = Vec::with_capacity(self.changes.len());
		for change_id in self.changes.iter() {
			//FIXME: Error handling
			let disk_change = database.get_change(change_id.clone()).unwrap();
			let change = try!(disk_change.from_disk(database));
			change_seq.push(change);
		}
		return Ok(change_seq);
	}
}


//How do I express this? Or do I need to have two different enums...
//type TreeChangeSeq<P> = VecChangeSeq<TreeChange<P, Self>>
//pub enum SomeEnum<T> {VarA(T), VarB, VarC}; fn main() {let foo: SomeEnum<Vec<Self>> = VarA(vec!(VarB, VarC)); }








