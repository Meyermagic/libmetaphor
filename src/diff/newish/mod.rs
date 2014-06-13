

pub trait TreeDiff<C: Change> {
	fn diff_trees<T1: Tree, T2: Tree>(old: T1, new: T2) -> C;
}

pub trait Diff<P: Patch> {
	fn diff_blobs<B1: Blob, B2: Blob>(old: B1, new: B2) -> P;
}

pub trait Change {
	fn patch_tree<T1: MutableTree>(&self, old: T1);
}

pub trait Patch {
	fn patch_blob<B1: MutableBlob>()
}