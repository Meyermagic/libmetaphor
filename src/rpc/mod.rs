
use std::io::IoResult;

pub use capnp::capability::CallContext;

pub use capnp_rpc::ez_rpc::{EzRpcClient, EzRpcServer};

pub use rpc::remote_capnp::ID;
pub use rpc::remote_capnp::Commit;
pub use rpc::remote_capnp::ChangeSeq;
pub use rpc::remote_capnp::Change;
pub use rpc::remote_capnp::Patch;
pub use rpc::remote_capnp::RemoteRepository;

pub use rpc::remote_capnp::RemoteRepository::{Server, Client};
pub use rpc::remote_capnp::RemoteRepository::{CloneContext, ChangesContext, ChildrenContext,
	                                            PushContext, TagContext, UntagContext, TaggedContext};
pub use rpc::remote_capnp::RemoteRepository::{CloneParams, CloneResults};
pub use rpc::remote_capnp::RemoteRepository::{ChangesParams, ChangesResults};
pub use rpc::remote_capnp::RemoteRepository::{ChildrenParams, ChildrenResults};
pub use rpc::remote_capnp::RemoteRepository::{PushParams, PushResults};
pub use rpc::remote_capnp::RemoteRepository::{TagParams, TagResults};
pub use rpc::remote_capnp::RemoteRepository::{UntagParams, UntagResults};
pub use rpc::remote_capnp::RemoteRepository::{TaggedParams, TaggedResults};

pub mod remote_capnp;


pub fn repository_server<S: Send+Server>(addr: &str, srv: Box<S>) -> IoResult<EzRpcServer> {
	let rpc_server = try!(EzRpcServer::new(addr));
	let repo_server = (box RemoteRepository::ServerDispatch{ server: srv }) as Box<::capnp::capability::Server:Send>;
	rpc_server.export_cap("remote_repository", repo_server);
	return Ok(rpc_server);
}

pub fn repository_client(addr: &str) -> IoResult<Client> {
	let mut rpc_client = try!(EzRpcClient::new(addr));
	let repo_client: Client = rpc_client.import_cap("remote_repository");
	return Ok(repo_client);
}




