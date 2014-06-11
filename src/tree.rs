use object::{Object, Hasher};
use blob::{Blob, MutableBlob};
use id::{ID, EmptyID, NullID};
use diff::{Diff, Patch, PatienceLinePatch, PatienceLineDiff};

use blob::{FsBlob, EmptyBlob};
use change::{Change, RecChange};
use change::{CreateTree, DeleteTree, MoveTree, CreateBlob, DeleteBlob, MoveBlob, ModifyBlob, ApplyChanges};


use std::collections::HashSet;

use std::io::fs::File;
use std::io::fs;
use std::io;
use std::io::IoResult;

pub trait Tree<B: Blob> {
	fn trees(&self) -> IoResult<Vec<Path>>;
	fn blobs(&self) -> IoResult<Vec<Path>>;
}

pub trait MutableTree<B: MutableBlob>: Tree<B> {
	fn create_tree(&mut self, path: &Path) -> IoResult<()>;
	fn delete_tree(&mut self, path: &Path) -> IoResult<()>;
	fn move_tree(&mut self, path_a: &Path, path_b: &Path) -> IoResult<()>;
	fn modify_tree<C: Change>(&mut self, path: &Path, change: &C) -> IoResult<()>;

	fn create_blob(&mut self, path: &Path) -> IoResult<()>;
	fn delete_blob(&mut self, path: &Path) -> IoResult<()>;
	fn move_blob(&mut self, path_a: &Path, path_b: &Path) -> IoResult<()>;
	fn modify_blob<P: Patch>(&mut self, path: &Path, patch: P) -> IoResult<()>;
}

pub trait FlatTree<B: Blob>: Tree<B> {
	fn get_tree(&self, path: &Path) -> IoResult<Self>;
	fn get_blob(&self, path: &Path) -> IoResult<B>;
}

pub trait MutableFlatTree<B: MutableBlob>: FlatTree<B>+MutableTree<B> {
	fn get_mut_tree(&mut self, path: &Path) -> IoResult<Self>;
	fn get_mut_blob(&mut self, path: &Path) -> IoResult<B>;
}

pub trait DeepTree<B: Blob>: Tree<B> {
	fn get_ref_tree<'a>(&'a self, path: &Path) -> IoResult<&'a Self>;
	fn get_ref_blob<'a>(&'a self, path: &Path) -> IoResult<&'a B>;
}

pub trait MutableDeepTree<B: Blob>: DeepTree<B>+MutableTree<B> {
	fn get_mut_ref_tree<'a>(&'a mut self, path: &Path) -> IoResult<&'a mut Self>;
	fn get_mut_ref_blob<'a>(&'a mut self, path: &Path) -> IoResult<&'a mut B>;
}

//pub trait ImmutableTree (should we have this? maybe Object only supported by immutable things?)
//probably unnecessary complication

//pub trait MonitorTree (for things that can change under our feet?)
//then we could enforce that we be dealing with a "safe" copy in functions. 


//Implementations

pub struct FsTree {
	path: Path
}

impl FsTree {
	pub fn new(path: &Path) -> IoResult<FsTree> {
		return Ok(FsTree {path: path.clone()});
	}
}

impl Tree<FsBlob> for FsTree {
	fn trees(&self) -> IoResult<Vec<Path>> {
		let mut tree_paths = vec!();
		for path in try!(fs::readdir(&self.path)).move_iter() {
			if path.is_dir() {
				tree_paths.push(path.path_relative_from(&self.path).unwrap());
			}
		}

		return Ok(tree_paths);
	}

	fn blobs(&self) -> IoResult<Vec<Path>> {
		let mut blob_paths = vec!();
		for path in try!(fs::readdir(&self.path)).move_iter() {
			if path.is_file() {
				blob_paths.push(path.path_relative_from(&self.path).unwrap());
			}
		}
		return Ok(blob_paths);
	}
}

impl MutableTree<FsBlob> for FsTree {
	fn create_tree(&mut self, path: &Path) -> IoResult<()> {
		fs::mkdir_recursive(&self.path.join(path), io::UserDir)
	}

	fn delete_tree(&mut self, path: &Path) -> IoResult<()> {
		fs::rmdir_recursive(&self.path.join(path))
	}

	fn move_tree(&mut self, path_a: &Path, path_b: &Path) -> IoResult<()> {
		fs::rename(&self.path.join(path_a), &self.path.join(path_b))
	}

	fn modify_tree<C: Change>(&mut self, path: &Path, change: &C) -> IoResult<()> {
		change.patch_tree(&mut try!(self.get_mut_tree(path)))
	}

	fn create_blob(&mut self, path: &Path) -> IoResult<()> {
		try!(File::create(&self.path.join(path)));
		return Ok(());
	}

	fn delete_blob(&mut self, path: &Path) -> IoResult<()> {
		fs::unlink(&self.path.join(path))
	}

	fn move_blob(&mut self, path_a: &Path, path_b: &Path) -> IoResult<()> {
		fs::rename(&self.path.join(path_a), &self.path.join(path_b))
	}

	fn modify_blob<P: Patch>(&mut self, path: &Path, patch: P) -> IoResult<()> {
		let mut blob = try!(self.get_mut_blob(path));
		return patch.patch_blob(&mut blob);
	}
}

impl FlatTree<FsBlob> for FsTree {
	fn get_tree(&self, path: &Path) -> IoResult<FsTree> {
		FsTree::new(&self.path.join(path)).map(|fstree| { fstree })
	}

	fn get_blob(&self, path: &Path) -> IoResult<FsBlob> {
		FsBlob::new(&self.path.join(path)).map(|fsblob| { fsblob })
	}
}

impl MutableFlatTree<FsBlob> for FsTree {
	fn get_mut_tree(&mut self, path: &Path) -> IoResult<FsTree> {
		FsTree::new(&self.path.join(path)).map(|fstree| { fstree })
	}
	fn get_mut_blob(&mut self, path: &Path) -> IoResult<FsBlob> {
		FsBlob::new(&self.path.join(path)).map(|fsblob| { fsblob })
	}
}

pub struct EmptyTree;

impl Tree<EmptyBlob> for EmptyTree {
	fn trees(&self) -> IoResult<Vec<Path>> {
		return Ok(vec!());
	}

	fn blobs(&self) -> IoResult<Vec<Path>> {
		return Ok(vec!());
	}
}

impl FlatTree<EmptyBlob> for EmptyTree {
	fn get_tree(&self, path: &Path) -> IoResult<EmptyTree> {
		return Err(io::standard_error(io::PathDoesntExist));
	}

	fn get_blob(&self, path: &Path) -> IoResult<EmptyBlob> {
		return Err(io::standard_error(io::FileNotFound));
	}
}

pub fn tree_diff<Ba: Blob, Bb: Blob, Ta: FlatTree<Ba>, Tb: FlatTree<Bb>>(a: Ta, b: Tb) -> IoResult<Vec<RecChange<Box<Patch>>>> {
	let mut changeseq = vec!();

	let old_trees: HashSet<Path> = try!(a.trees()).move_iter().collect();
	let new_trees: HashSet<Path> = try!(b.trees()).move_iter().collect();

	let mut only_old_trees = old_trees.difference(&new_trees);
	let mut only_new_trees = new_trees.difference(&old_trees);
	let mut both_trees = old_trees.intersection(&new_trees);

	for tree in only_old_trees {
		changeseq.push(DeleteTree(tree.clone()));
	}

	for tree in only_new_trees {
		changeseq.push(CreateTree(tree.clone()));
		//FIXME: error handling
		let inner_changes = try!(tree_diff(EmptyTree, try!(b.get_tree(tree))));

		if inner_changes.len() > 0 {
			changeseq.push(ApplyChanges(tree.clone(), inner_changes));
		}
	}

	for tree in both_trees {
		let old_tree = try!(a.get_tree(tree));
		let new_tree = try!(b.get_tree(tree));

		let inner_changes = try!(tree_diff(old_tree, new_tree));
		if inner_changes.len() > 0 {
			changeseq.push(ApplyChanges(tree.clone(), inner_changes));
		}
	}

	let old_blobs: HashSet<Path> = try!(a.blobs()).move_iter().collect();
	let new_blobs: HashSet<Path> = try!(b.blobs()).move_iter().collect();

	let mut only_old_blobs = old_blobs.difference(&new_blobs);
	let mut only_new_blobs = new_blobs.difference(&old_blobs);
	let mut both_blobs = old_blobs.intersection(&new_blobs);

	for blob in only_old_blobs {
		changeseq.push(DeleteBlob(blob.clone()));
	}

	for blob in only_new_blobs {
		changeseq.push(CreateBlob(blob.clone()));
		let differ = PatienceLineDiff;
		let patch: PatienceLinePatch = try!(differ.diff_blobs(&EmptyBlob, &try!(b.get_blob(blob))));
		changeseq.push(ModifyBlob(blob.clone(), box patch as Box<Patch>))
	}

	for blob in both_blobs {
		let old_blob = try!(a.get_blob(blob));
		let new_blob = try!(b.get_blob(blob));

		let differ = PatienceLineDiff;
		let patch: PatienceLinePatch = try!(differ.diff_blobs(&old_blob, &new_blob));
		changeseq.push(ModifyBlob(blob.clone(), box patch as Box<Patch>));
	}

	return Ok(changeseq);
}