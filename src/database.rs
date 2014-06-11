
use id::ID;


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

	fn commit_history(&mut self, id: ID) -> Option<Vec<ID>> {

	}
}


pub struct TrivialObjectDb<T> {
	path: Path,
	dirty: bool,
	cache: HashMap<ID, T>,
}

impl<T: Disk> TrivialObjectDb<T> {
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
		let mut file = try!(io::File::open(self.path));
		let object_count = try!(file.read_le_u64());

		self.cache.reserve(object_count as uint);

		for _ in range(0, object_count) {
			let chunk_size = try!(file.read_le_u64());
			let mut chunk = try!(file.read_exact(chunk_size as uint));

			let mut reader = BufReader::new(chunk.as_slice());
			let json_object = json::from_reader(&mut reader as &mut Reader).unwrap();
			let mut decoder = Json::Decoder::new(json_object);
			let object: T = Decodable::decode(&mut decoder).unwrap();
			self.cache.insert(object.id(), object);
		}

		self.dirty = false;
		return Ok(());
	}

	pub fn flush(&mut self) -> IoResult<()> {
		let mut file = try!(io::File::create(path));

		let object_count = self.cache.len();
		try!(file.write_le_u64(object_count as u64));

		for v in self.cache.values() {
			let buffer = json::Encoder::buffer_encode(v);
			let chunk_size = buffer.len();
			try!(file.write_le_u64(chunk_size as u64));
			try!(file.write(buffer.as_slice()));
		}

		self.dirty = false;
		return Ok(());
	}

	pub fn set(&mut self, object: T) -> bool {
		self.dirty = true;
		!self.cache.insert(object.id(), object)
	}

	pub fn get(&mut self, id: ID) -> Option<T> {
		self.cache.find_copy(&id)
	}

	pub fn unset(&mut self, id: ID) -> bool {
		self.dirty = true;
		self.cache.remove(&id)
	}
}

/*
// Do we want to do this? Might be surprising.
impl<T: Disk> Drop for TrivialObjectDb<T> {
	fn drop(&mut self) {
		if self.dirty {
			self.store();
		}
	}
}
*/

pub struct TrivialDb {
	tag_db: TrivialObjectDb<DiskTag>,
	commit_db: TrivialObjectDb<DiskCommit>,
	changeseq_db: TrivialObjectDb<DiskChangeSeq>,
	change_db: TrivialObjectDb<DiskChange>,
	patch_db: TrivialObjectDb<DiskPatch>,
}

impl TrivialDb {
	pub fn new(met_root: &Path) -> TrivialDb;
}

impl Database for TrivialDb {
	fn get_tag(&mut self, id: ID) -> Option<DiskTag> { self.tag_db.get(id) }
	fn get_commit(&mut self, id: ID) -> Option<DiskCommit> { self.commit_db.get(id) }
	fn get_changeseq(&mut self, id: ID) -> Option<DiskChangeSeq> { self.changeseq_db.get(id) }
	fn get_change(&mut self, id: ID) -> Option<DiskChange> { self.change_db.get(id) }
	fn get_patch(&mut self, id: ID) -> Option<DiskPatch> { self.patch_db.get(id) }

	fn set_tag(&mut self, tag: DiskTag) -> bool { self.tag_db.set(tag) }
	fn set_commit(&mut self, commit: DiskCommit) -> bool { self.commit_db.set(commit) }
	fn set_changeseq(&mut self, seq: DiskChangeSeq) -> bool { self.changeseq_db.set(changeseq) }
	fn set_change(&mut self, change: DiskChange) -> bool { self.change_db.set(change) }
	fn set_patch(&mut self, patch: DiskPatch) -> bool { self.patch_db.set(patch) }

	fn drop_tag(&mut self, id: ID) -> bool { self.tag_db.unset(id) }
	fn drop_commit(&mut self, id: ID) -> bool { self.commit_db.unset(id) }
	fn drop_changeseq(&mut self, id: ID) -> bool { self.changeseq_db.unset(id) }
	fn drop_change(&mut self, id: ID) -> bool { self.change_db.unset(id) }
	fn drop_patch(&mut self, id: ID) -> bool { self.patch_db.unset(id) }
}

