


pub type MetResult<T> = Result<T, MetError>;

pub struct MetError {
	pub kind: MetErrorKind,
	pub desc: &'static str,
	pub detail: Option<~str>,
}

pub enum MetErrorKind {
	IoError(IoError),
	
}

