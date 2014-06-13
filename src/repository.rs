use database::Database;


pub trait Repository {
}

pub struct FsRepo {
	root: Path
}

pub fn find_repo_root(path: &Path) -> Option<Path> {
	let mut path = path.clone();
	loop {
		if path.join(".met").is_dir() {
			return Some(path);
		}
		let done = !path.pop();
		if done {
			return None;
		}
	}
}

pub fn find_repo(path: &Path) -> Option<FsRepo> {
	match find_repo_root(path) {
		Some(path) => Some(FsRepo{root: path}),
		None => None,
	}
}