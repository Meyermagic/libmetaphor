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

extern crate collections;
extern crate serialize;
extern crate semver;

// Re-exports
//TODO: What should be exported here?
pub use id::ID;
pub use object::Object;

// Modules
pub mod blob;
pub mod change;
pub mod changeseq;
pub mod commit;
pub mod diff;
pub mod disk;
pub mod id;
pub mod object;
pub mod store;
pub mod tag;
pub mod tree;
pub mod macros;
