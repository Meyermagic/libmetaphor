#![crate_id = "libmetaphor"]
#![crate_type = "rlib"]

#![feature(phase)]
#![feature(macro_rules)]


//#![deny(missing_doc)]

//! libmetaphor is a library for handling Metaphor VCS repositories.

#[phase(plugin, link)]
extern crate log;

extern crate crypto = "rust-crypto";
extern crate sqlite = "sqlite3";
extern crate libbspatch = "bspatch";
extern crate libbsdiff = "bsdiff";
extern crate capnp;
extern crate capnp_rpc = "capnp-rpc";

extern crate collections;
extern crate serialize;
extern crate semver;

// Re-exports
//TODO: What should be exported here?
pub use id::{ID, EmptyID, NullID};
pub use object::Object;
pub use tag::{Tag, DiskTag};
pub use database::{Database, TrivialDb};
pub use disk::{Disk, ToDisk, FromDisk};
pub use commit::{DiskCommit, CommitType, MemCommit};
pub use change::{Change, DiskChange, DiskChangeSeq, RecChange};
pub use tree::{Tree, MutableTree, FlatTree, MutableFlatTree, DeepTree, MutableDeepTree, FsTree};
pub use blob::{Blob, MutableBlob, FsBlob};
pub use diff::{Diff, Patch, DiskPatch};

// Modules
pub mod blob;
pub mod change;
//pub mod changeseq;
pub mod commit;
pub mod diff;
pub mod disk;
pub mod id;
pub mod object;
pub mod store;
pub mod tag;
pub mod tree;
pub mod macros;
pub mod repository;
pub mod database;
pub mod rpc;
//Things to fix in rust:
//.as_vec vs .as_bytes
//Actually submit that grapheme cluster iteration pr
//Documentation for core::int::BITS uses architecture of rustdoc's server
//http://doc.rust-lang.org/std/owned/trait.AnyOwnExt.html uses ~
//Fix RWLock -> RwLock according to consistency guidelines
