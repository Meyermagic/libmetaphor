

@0xd6b6543239891b03;

struct ID {
	bytes @0 :Data;
}

struct Commit {
	id @0 :ID;
	body @1 :Data;
}

struct ChangeSeq {
	id @0 :ID;
	body @1 :Data;
}

struct Change {
	id @0 :ID;
	body @1 :Data;
}

struct Patch {
	id @0 :ID;
	body @1 :Data;
}

interface RemoteRepository {
	# Get a list of all commits in the repository
	clone @0 () -> (commits :List(Commit));

	# Get changes corresponding to commit id, relevant to a subdirectory
	changes @1 (id :ID, path :Text = ".") -> (changeseqs :List(ChangeSeq), changes :List(Change), patches :List(Patch));

	# Get commits made on top of a commit ID
	children @2 (id :ID) -> (commits :List(Commit));

	# Push a new commit, along with changes
	push @3 (commit :Commit, changeseqs :List(ChangeSeq), changes :List(Change), patches :List(Patch));

	# Tag a specified ID
	tag @4 (key :Text, value :Text, id :ID) -> (result :Bool);

	# Untag a specified ID
	untag @5 (key :Text, value :Text, id :ID) -> (result :Bool);

	# Get IDs tagged by a tag
	tagged @6 (key :Text, value :Text) -> (objects :List(ID));
}

