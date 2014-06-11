use object::{Object, Hasher};
use tree::{Tree, MutableTree};
use blob::{Blob, MutableBlob};
use id::{ID, EmptyID, NullID};
use diff::{Diff, Patch};

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

type DiskChange = FlatChange<ID, ID>;

pub struct VecChangeSeq<T> {
	changes: Vec<T>
}

type DiskChangeSeq = VecChangeSeq<ID>;



//How do I express this? Or do I need to have two different enums...
//type TreeChangeSeq<P> = VecChangeSeq<TreeChange<P, Self>>
//pub enum SomeEnum<T> {VarA(T), VarB, VarC}; fn main() {let foo: SomeEnum<Vec<Self>> = VarA(vec!(VarB, VarC)); }








