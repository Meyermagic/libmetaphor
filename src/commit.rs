

use id::ID;

pub type CommitID = ID;

#[deriving(Show,Clone,Encodable,Decodable)]
pub enum CommitType {
	Basic(CommitID),
	Merge(CommitID, CommitID)
}

#[deriving(Show,Clone,Encodable,Decodable)]
pub struct DiskCommit {
	author: String,
	short: String,
	long: String,
	family: CommitType,
	changes: ChangeSeqID,
}
