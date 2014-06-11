use database::Database;

pub struct FsRepo {
	root: Path
}


pub trait Repository<'a, D: Database> {
	fn get_database<'a>()
}
