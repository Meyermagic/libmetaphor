
use id::{ID, EmptyID};

// The structures we'll be storing
use tag::DiskTag;
use commit::DiskCommit;
use change::{DiskChange, DiskChangeSeq};
use diff::DiskPatch;
use disk::Disk;
use object::Object;

use tag::tag_id;

use std::collections::HashMap;
use std::io::fs;
use std::io;
use std::io::{IoResult, BufReader, File, MemWriter};
use std::io::{BufferedWriter, BufferedReader};

use commit::Basic;

pub trait Database {
	fn get_tag(&self, id: ID) -> Option<DiskTag>;
	fn get_commit(&self, id: ID) -> Option<DiskCommit>;
	fn get_changeseq(&self, id: ID) -> Option<DiskChangeSeq>;
	fn get_change(&self, id: ID) -> Option<DiskChange>;
	fn get_patch(&self, id: ID) -> Option<DiskPatch>;

	fn set_tag(&mut self, tag: DiskTag) -> bool;
	fn set_commit(&mut self, commit: DiskCommit) -> bool;
	fn set_changeseq(&mut self, seq: DiskChangeSeq) -> bool;
	fn set_change(&mut self, change: DiskChange) -> bool;
	fn set_patch(&mut self, patch: DiskPatch) -> bool;

	fn drop_tag(&mut self, id: ID) -> bool;
	fn drop_commit(&mut self, id: ID) -> bool;
	fn drop_changeseq(&mut self, id: ID) -> bool;
	fn drop_change(&mut self, id: ID) -> bool;
	fn drop_patch(&mut self, id: ID) -> bool;

	fn forward_refs(&self, id: ID) -> Option<Vec<ID>>;
	fn backward_refs(&self, id: ID) -> Option<Vec<ID>>;

	fn commit_history(&mut self, id: ID) -> Option<Vec<ID>> {
		let mut history: Vec<ID> = vec!();
		let mut current_id: ID = id;
		loop {
			history.push(current_id);
			let disk_commit = self.get_commit(current_id).unwrap();
			match disk_commit.family {
				Basic(some_id) => {
					if some_id == EmptyID {
						history.reverse();
						return Some(history);
					}
					current_id = some_id;
				},
				_ => {
					error!("merge not yet implemented in commit history");
					return None;
				}
			}
		}
		unreachable!();
	}

	fn tag_object(&mut self, key: &str, value: &str, id: ID) {
		let mut existing = match self.get_tag(tag_id(key, value)) {
			Some(tag) => tag,
			None => DiskTag{key: String::from_str(key), value: String::from_str(value), targets: vec!()},
		};

		//FIXME: Should we be able to double tag things? This lets you.
		existing.targets.push(id);

		self.set_tag(existing);
	}

	fn untag_object(&mut self, key: &str, value: &str, id: ID) {
		let mut existing = match self.get_tag(tag_id(key, value)) {
			Some(tag) => tag,
			None => DiskTag{key: String::from_str(key), value: String::from_str(value), targets: vec!()},
		};

		existing.targets.retain(|t| t != &id);

		//FIXME: Should we remove tags that point to nothing?
		// or maybe modify get_tag to never return None, only empty tags
		// (so you can pretend they always exist)

		self.set_tag(existing);
	}
}


pub struct TrivialObjectDb<T> {
	path: Path,
	dirty: bool,
	cache: HashMap<ID, T>,
}

impl<T: Disk+Object+Clone> TrivialObjectDb<T> {
	pub fn new(db_file: &Path) -> TrivialObjectDb<T> {
		let mut db = TrivialObjectDb {
			path: db_file.clone(),
			dirty: false,
			cache: HashMap::new(),
		};

		db.load();

		return db;
	}

	fn load(&mut self) -> IoResult<()> {
		let mut file = try!(io::File::open(&self.path));
		let object_count = try!(file.read_le_u64());

		self.cache.reserve(object_count as uint);

		for _ in range(0, object_count) {
			let chunk_size = try!(file.read_le_u64());
			let mut chunk = try!(file.read_exact(chunk_size as uint));

			let mut reader = BufReader::new(chunk.as_slice());
			let object: T = try!(Disk::read(&mut reader));
			self.cache.insert(object.id(), object);
		}

		self.dirty = false;
		return Ok(());
	}

	pub fn flush(&mut self) -> IoResult<()> {
		let mut file = try!(io::File::create(&self.path));

		let object_count = self.cache.len();
		try!(file.write_le_u64(object_count as u64));

		for v in self.cache.values() {
			let mut memwriter = MemWriter::new();
			try!(v.write(&mut memwriter));
			let buffer = memwriter.get_ref();
			let chunk_size = buffer.len();
			try!(file.write_le_u64(chunk_size as u64));
			try!(file.write(buffer));
		}

		self.dirty = false;
		return Ok(());
	}

	pub fn set(&mut self, object: T) -> bool {
		self.dirty = true;
		!self.cache.insert(object.id(), object)
	}

	pub fn get(&self, id: ID) -> Option<T> {
		self.cache.find_copy(&id)
	}

	pub fn unset(&mut self, id: ID) -> bool {
		self.dirty = true;
		self.cache.remove(&id)
	}
}


struct TrivialHistoryDb {
	forward_path: Path,
	forward_cache: HashMap<ID, Vec<ID>>,
	backward_path: Path,
	backward_cache: HashMap<ID, Vec<ID>>,
	dirty: bool,
}

impl TrivialHistoryDb {
	pub fn new(forward: &Path, backward: &Path) -> TrivialHistoryDb {
		let mut db = TrivialHistoryDb {
			forward_path: forward.clone(),
			forward_cache: HashMap::new(),
			backward_path: backward.clone(),
			backward_cache: HashMap::new(),
			dirty: false,
		};
		db.load().unwrap();
		return db;
	}

	fn load(&mut self) -> IoResult<()> {
		try!(self.load_forward());
		try!(self.load_backward());
		return Ok(());
	}

	pub fn flush(&mut self) -> IoResult<()> {
		try!(self.flush_forward());
		try!(self.flush_backward());
		self.dirty = false;
		return Ok(());
	}

	fn load_forward(&mut self) -> IoResult<()> {
		let mut file = try!(io::File::open(&self.forward_path));
		let object_count = try!(file.read_le_u64());
		
		self.forward_cache.reserve(object_count as uint);

		for _ in range(0, object_count) {
			let key_id: ID = try!(Disk::read(&mut file));

			let ref_count = try!(file.read_le_u64()) as uint;
			let mut value_ids = Vec::with_capacity(ref_count);

			for _ in range(0, ref_count) {
				let value_id: ID = try!(Disk::read(&mut file));
				value_ids.push(value_id);
			}
			self.forward_cache.insert(key_id, value_ids);
		}

		return Ok(());
	}

	fn load_backward(&mut self) -> IoResult<()> {
		let mut file = try!(io::File::open(&self.backward_path));
		let object_count = try!(file.read_le_u64());
		
		self.backward_cache.reserve(object_count as uint);

		for _ in range(0, object_count) {
			let key_id: ID = try!(Disk::read(&mut file));

			let ref_count = try!(file.read_le_u64()) as uint;
			let mut value_ids = Vec::with_capacity(ref_count);

			for _ in range(0, ref_count) {
				let value_id: ID = try!(Disk::read(&mut file));
				value_ids.push(value_id);
			}
			self.backward_cache.insert(key_id, value_ids);
		}
		
		return Ok(());
	}

	pub fn flush_forward(&mut self) -> IoResult<()> {
		let mut file = try!(io::File::create(&self.forward_path));

		let object_count = self.forward_cache.len();
		try!(file.write_le_u64(object_count as u64));

		let mut buffered = BufferedWriter::new(file);
		for (k, v) in self.forward_cache.iter() {
			let ref_count = v.len() as u64;
			try!(k.write(&mut buffered));
			try!(buffered.write_le_u64(ref_count));
			for id in v.iter() {
				try!(id.write(&mut buffered));
			}
		}

		self.dirty = false;
		return Ok(());
	}

	pub fn flush_backward(&mut self) -> IoResult<()> {
		let mut file = try!(io::File::create(&self.backward_path));

		let object_count = self.backward_cache.len();
		try!(file.write_le_u64(object_count as u64));

		let mut buffered = BufferedWriter::new(file);
		for (k, v) in self.backward_cache.iter() {
			let ref_count = v.len() as u64;
			try!(k.write(&mut buffered));
			try!(buffered.write_le_u64(ref_count));
			for id in v.iter() {
				try!(id.write(&mut buffered));
			}
		}

		self.dirty = false;
		return Ok(());
	}
}


pub struct TrivialDb {
	tag_db: TrivialObjectDb<DiskTag>,
	commit_db: TrivialObjectDb<DiskCommit>,
	changeseq_db: TrivialObjectDb<DiskChangeSeq>,
	change_db: TrivialObjectDb<DiskChange>,
	patch_db: TrivialObjectDb<DiskPatch>,
	history_db: TrivialHistoryDb,
}

impl TrivialDb {
	pub fn new(met_root: &Path) -> TrivialDb {
		TrivialDb {
			tag_db: TrivialObjectDb::new(&met_root.join("tag.db")),
			commit_db: TrivialObjectDb::new(&met_root.join("commit.db")),
			changeseq_db: TrivialObjectDb::new(&met_root.join("changeseq.db")),
			change_db: TrivialObjectDb::new(&met_root.join("change.db")),
			patch_db: TrivialObjectDb::new(&met_root.join("patch.db")),
			history_db: TrivialHistoryDb::new(&met_root.join("forward.db"), &met_root.join("backward.db")),
		}
	}

	fn add_refs(&self, parent: ID, child: ID) {
		unimplemented!();
	}

	fn remove_refs(&self, commit: ID) {
		unimplemented!();
	}
}

impl Drop for TrivialDb {
	fn drop(&mut self) {
		self.tag_db.flush();
		self.commit_db.flush();
		self.changeseq_db.flush();
		self.change_db.flush();
		self.patch_db.flush();
	}
}


impl Database for TrivialDb {
	fn get_tag(&self, id: ID) -> Option<DiskTag> { self.tag_db.get(id) }
	fn get_commit(&self, id: ID) -> Option<DiskCommit> { self.commit_db.get(id) }
	fn get_changeseq(&self, id: ID) -> Option<DiskChangeSeq> { self.changeseq_db.get(id) }
	fn get_change(&self, id: ID) -> Option<DiskChange> { self.change_db.get(id) }
	fn get_patch(&self, id: ID) -> Option<DiskPatch> { self.patch_db.get(id) }

	fn set_tag(&mut self, tag: DiskTag) -> bool { self.tag_db.set(tag) }
	fn set_commit(&mut self, commit: DiskCommit) -> bool { self.commit_db.set(commit) }
	fn set_changeseq(&mut self, changeseq: DiskChangeSeq) -> bool { self.changeseq_db.set(changeseq) }
	fn set_change(&mut self, change: DiskChange) -> bool { self.change_db.set(change) }
	fn set_patch(&mut self, patch: DiskPatch) -> bool { self.patch_db.set(patch) }

	fn drop_tag(&mut self, id: ID) -> bool { self.tag_db.unset(id) }
	fn drop_commit(&mut self, id: ID) -> bool { self.commit_db.unset(id) }
	fn drop_changeseq(&mut self, id: ID) -> bool { self.changeseq_db.unset(id) }
	fn drop_change(&mut self, id: ID) -> bool { self.change_db.unset(id) }
	fn drop_patch(&mut self, id: ID) -> bool { self.patch_db.unset(id) }

	fn forward_refs(&self, id: ID) -> Option<Vec<ID>> {
		unimplemented!();
	}

	fn backward_refs(&self, id: ID) -> Option<Vec<ID>> {
		unimplemented!();
	}
}

