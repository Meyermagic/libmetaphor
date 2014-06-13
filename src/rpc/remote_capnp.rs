#![allow(unused_imports)]
#![allow(dead_code)]

pub mod ID {
  use std;
  use capnp::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
  use capnp::list::ToU16;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 0, pointers : 1, preferred_list_encoding : layout::Pointer};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_bytes(&self) -> Data::Reader<'a> {
      self.reader.get_pointer_field(0).get_data(std::ptr::null(), 0)
    }
    pub fn has_bytes(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_bytes(&self) -> Data::Builder<'a> {
      self.builder.get_pointer_field(0).get_data(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_bytes(&self, value : Data::Reader) {
      self.builder.get_pointer_field(0).set_data(value);
    }
    #[inline]
    pub fn init_bytes(&self, size : uint) -> Data::Builder<'a> {
      self.builder.get_pointer_field(0).init_data(size)
    }
    pub fn has_bytes(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
  }

  pub struct Pipeline { _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
  }
}

pub mod Commit {
  use std;
  use capnp::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
  use capnp::list::ToU16;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 0, pointers : 2, preferred_list_encoding : layout::InlineComposite};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
      FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
    }
    pub fn has_id(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Reader<'a> {
      self.reader.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    pub fn has_body(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
    }
    #[inline]
    pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
      self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
    }
    #[inline]
    pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
    }
    pub fn has_id(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_body(&self, value : Data::Reader) {
      self.builder.get_pointer_field(1).set_data(value);
    }
    #[inline]
    pub fn init_body(&self, size : uint) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).init_data(size)
    }
    pub fn has_body(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
  }

  pub struct Pipeline { _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
  }
}

pub mod ChangeSeq {
  use std;
  use capnp::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
  use capnp::list::ToU16;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 0, pointers : 2, preferred_list_encoding : layout::InlineComposite};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
      FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
    }
    pub fn has_id(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Reader<'a> {
      self.reader.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    pub fn has_body(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
    }
    #[inline]
    pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
      self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
    }
    #[inline]
    pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
    }
    pub fn has_id(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_body(&self, value : Data::Reader) {
      self.builder.get_pointer_field(1).set_data(value);
    }
    #[inline]
    pub fn init_body(&self, size : uint) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).init_data(size)
    }
    pub fn has_body(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
  }

  pub struct Pipeline { _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
  }
}

pub mod Change {
  use std;
  use capnp::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
  use capnp::list::ToU16;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 0, pointers : 2, preferred_list_encoding : layout::InlineComposite};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
      FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
    }
    pub fn has_id(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Reader<'a> {
      self.reader.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    pub fn has_body(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
    }
    #[inline]
    pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
      self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
    }
    #[inline]
    pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
    }
    pub fn has_id(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_body(&self, value : Data::Reader) {
      self.builder.get_pointer_field(1).set_data(value);
    }
    #[inline]
    pub fn init_body(&self, size : uint) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).init_data(size)
    }
    pub fn has_body(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
  }

  pub struct Pipeline { _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
  }
}

pub mod Patch {
  use std;
  use capnp::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
  use capnp::list::ToU16;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 0, pointers : 2, preferred_list_encoding : layout::InlineComposite};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
      FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
    }
    pub fn has_id(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Reader<'a> {
      self.reader.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    pub fn has_body(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
    }
    #[inline]
    pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
      self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
    }
    #[inline]
    pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
    }
    pub fn has_id(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_body(&self) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_body(&self, value : Data::Reader) {
      self.builder.get_pointer_field(1).set_data(value);
    }
    #[inline]
    pub fn init_body(&self, size : uint) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).init_data(size)
    }
    pub fn has_body(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
  }

  pub struct Pipeline { _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
    pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
  }
}

pub mod RemoteRepository {
  #![allow(unused_variable)]
  use capnp::AnyPointer;
  use capnp::capability::{ClientHook, FromClientHook, FromServer, Request, ServerHook};
  use capnp::capability;

  pub type CloneContext<'a> = capability::CallContext<CloneParams::Reader<'a>, CloneResults::Builder<'a>>;
  pub type ChangesContext<'a> = capability::CallContext<ChangesParams::Reader<'a>, ChangesResults::Builder<'a>>;
  pub type ChildrenContext<'a> = capability::CallContext<ChildrenParams::Reader<'a>, ChildrenResults::Builder<'a>>;
  pub type PushContext<'a> = capability::CallContext<PushParams::Reader<'a>, PushResults::Builder<'a>>;
  pub type TagContext<'a> = capability::CallContext<TagParams::Reader<'a>, TagResults::Builder<'a>>;
  pub type UntagContext<'a> = capability::CallContext<UntagParams::Reader<'a>, UntagResults::Builder<'a>>;
  pub type TaggedContext<'a> = capability::CallContext<TaggedParams::Reader<'a>, TaggedResults::Builder<'a>>;

  pub struct Client{ pub client : capability::Client }
  impl FromClientHook for Client {
    fn new(hook : Box<ClientHook:Send>) -> Client {
      Client { client : capability::Client::new(hook) }
    }
  }
  impl <T:ServerHook, U : Server + Send> FromServer<T,U> for Client {
    fn new(_hook : Option<T>, server : Box<U>) -> Client {
      Client { client : ServerHook::new_client(None::<T>, box ServerDispatch { server : server})}
    }
  }
  impl Clone for Client {
    fn clone(&self) -> Client {
      Client { client : capability::Client::new(self.client.hook.copy()) }
    }
  }
  impl Client {
    pub fn clone_request(&self) -> Request<CloneParams::Builder,CloneResults::Reader,CloneResults::Pipeline> {
      self.client.new_call(0x9dfa37bd5141803a, 0, None)
    }
    pub fn changes_request(&self) -> Request<ChangesParams::Builder,ChangesResults::Reader,ChangesResults::Pipeline> {
      self.client.new_call(0x9dfa37bd5141803a, 1, None)
    }
    pub fn children_request(&self) -> Request<ChildrenParams::Builder,ChildrenResults::Reader,ChildrenResults::Pipeline> {
      self.client.new_call(0x9dfa37bd5141803a, 2, None)
    }
    pub fn push_request(&self) -> Request<PushParams::Builder,PushResults::Reader,PushResults::Pipeline> {
      self.client.new_call(0x9dfa37bd5141803a, 3, None)
    }
    pub fn tag_request(&self) -> Request<TagParams::Builder,TagResults::Reader,TagResults::Pipeline> {
      self.client.new_call(0x9dfa37bd5141803a, 4, None)
    }
    pub fn untag_request(&self) -> Request<UntagParams::Builder,UntagResults::Reader,UntagResults::Pipeline> {
      self.client.new_call(0x9dfa37bd5141803a, 5, None)
    }
    pub fn tagged_request(&self) -> Request<TaggedParams::Builder,TaggedResults::Reader,TaggedResults::Pipeline> {
      self.client.new_call(0x9dfa37bd5141803a, 6, None)
    }
  }
  pub trait Server  {
    fn clone(&mut self, CloneContext);
    fn changes(&mut self, ChangesContext);
    fn children(&mut self, ChildrenContext);
    fn push(&mut self, PushContext);
    fn tag(&mut self, TagContext);
    fn untag(&mut self, UntagContext);
    fn tagged(&mut self, TaggedContext);
  }
  pub struct ServerDispatch<T> {
    pub server : Box<T>,
  }
  impl <T : Server> capability::Server for ServerDispatch<T> {
    fn dispatch_call(&mut self, interface_id : u64, method_id : u16, context : capability::CallContext<AnyPointer::Reader, AnyPointer::Builder>) {
      match interface_id {
        0x9dfa37bd5141803a => ServerDispatch::<T>::dispatch_call_internal(self.server, method_id, context),
        _ => {}
      }
    }
  }
  impl <T : Server> ServerDispatch<T> {
    pub fn dispatch_call_internal(server :&mut T, method_id : u16, context : capability::CallContext<AnyPointer::Reader, AnyPointer::Builder>) {
      match method_id {
        0 => server.clone(capability::internal_get_typed_context(context)),
        1 => server.changes(capability::internal_get_typed_context(context)),
        2 => server.children(capability::internal_get_typed_context(context)),
        3 => server.push(capability::internal_get_typed_context(context)),
        4 => server.tag(capability::internal_get_typed_context(context)),
        5 => server.untag(capability::internal_get_typed_context(context)),
        6 => server.tagged(capability::internal_get_typed_context(context)),
        _ => {}
      }
    }
  }

  pub mod CloneParams {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 0, preferred_list_encoding : layout::Void};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod CloneResults {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 1, preferred_list_encoding : layout::Pointer};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_commits(&self) -> StructList::Reader<'a,::rpc::remote_capnp::Commit::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(0).get_list(::rpc::remote_capnp::Commit::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_commits(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_commits(&self) -> StructList::Builder<'a,::rpc::remote_capnp::Commit::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(0).get_struct_list(::rpc::remote_capnp::Commit::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_commits(&self, value : StructList::Reader<'a,::rpc::remote_capnp::Commit::Reader<'a>>) {
        self.builder.get_pointer_field(0).set_list(&value.reader)
      }
      #[inline]
      pub fn init_commits(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::Commit::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::Commit::Builder<'a>>::new(
          self.builder.get_pointer_field(0).init_struct_list(size, ::rpc::remote_capnp::Commit::STRUCT_SIZE))
      }
      pub fn has_commits(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod ChangesParams {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 2, preferred_list_encoding : layout::InlineComposite};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
        FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
      }
      pub fn has_id(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_path(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      pub fn has_path(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
        self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
      }
      #[inline]
      pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
      }
      pub fn has_id(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_path(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_path(&self, value : Text::Reader) {
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_path(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_path(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
        FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
      }
    }
  }

  pub mod ChangesResults {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 3, preferred_list_encoding : layout::InlineComposite};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_changeseqs(&self) -> StructList::Reader<'a,::rpc::remote_capnp::ChangeSeq::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(0).get_list(::rpc::remote_capnp::ChangeSeq::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_changeseqs(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_changes(&self) -> StructList::Reader<'a,::rpc::remote_capnp::Change::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(1).get_list(::rpc::remote_capnp::Change::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_changes(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_patches(&self) -> StructList::Reader<'a,::rpc::remote_capnp::Patch::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(2).get_list(::rpc::remote_capnp::Patch::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_patches(&self) -> bool {
        !self.reader.get_pointer_field(2).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_changeseqs(&self) -> StructList::Builder<'a,::rpc::remote_capnp::ChangeSeq::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(0).get_struct_list(::rpc::remote_capnp::ChangeSeq::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_changeseqs(&self, value : StructList::Reader<'a,::rpc::remote_capnp::ChangeSeq::Reader<'a>>) {
        self.builder.get_pointer_field(0).set_list(&value.reader)
      }
      #[inline]
      pub fn init_changeseqs(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::ChangeSeq::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::ChangeSeq::Builder<'a>>::new(
          self.builder.get_pointer_field(0).init_struct_list(size, ::rpc::remote_capnp::ChangeSeq::STRUCT_SIZE))
      }
      pub fn has_changeseqs(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_changes(&self) -> StructList::Builder<'a,::rpc::remote_capnp::Change::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(1).get_struct_list(::rpc::remote_capnp::Change::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_changes(&self, value : StructList::Reader<'a,::rpc::remote_capnp::Change::Reader<'a>>) {
        self.builder.get_pointer_field(1).set_list(&value.reader)
      }
      #[inline]
      pub fn init_changes(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::Change::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::Change::Builder<'a>>::new(
          self.builder.get_pointer_field(1).init_struct_list(size, ::rpc::remote_capnp::Change::STRUCT_SIZE))
      }
      pub fn has_changes(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_patches(&self) -> StructList::Builder<'a,::rpc::remote_capnp::Patch::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(2).get_struct_list(::rpc::remote_capnp::Patch::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_patches(&self, value : StructList::Reader<'a,::rpc::remote_capnp::Patch::Reader<'a>>) {
        self.builder.get_pointer_field(2).set_list(&value.reader)
      }
      #[inline]
      pub fn init_patches(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::Patch::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::Patch::Builder<'a>>::new(
          self.builder.get_pointer_field(2).init_struct_list(size, ::rpc::remote_capnp::Patch::STRUCT_SIZE))
      }
      pub fn has_patches(&self) -> bool {
        !self.builder.get_pointer_field(2).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod ChildrenParams {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 1, preferred_list_encoding : layout::Pointer};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
        FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
      }
      pub fn has_id(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
        self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
      }
      #[inline]
      pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
      }
      pub fn has_id(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
        FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
      }
    }
  }

  pub mod ChildrenResults {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 1, preferred_list_encoding : layout::Pointer};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_commits(&self) -> StructList::Reader<'a,::rpc::remote_capnp::Commit::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(0).get_list(::rpc::remote_capnp::Commit::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_commits(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_commits(&self) -> StructList::Builder<'a,::rpc::remote_capnp::Commit::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(0).get_struct_list(::rpc::remote_capnp::Commit::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_commits(&self, value : StructList::Reader<'a,::rpc::remote_capnp::Commit::Reader<'a>>) {
        self.builder.get_pointer_field(0).set_list(&value.reader)
      }
      #[inline]
      pub fn init_commits(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::Commit::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::Commit::Builder<'a>>::new(
          self.builder.get_pointer_field(0).init_struct_list(size, ::rpc::remote_capnp::Commit::STRUCT_SIZE))
      }
      pub fn has_commits(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod PushParams {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 4, preferred_list_encoding : layout::InlineComposite};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_commit(&self) -> ::rpc::remote_capnp::Commit::Reader<'a> {
        FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
      }
      pub fn has_commit(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_changeseqs(&self) -> StructList::Reader<'a,::rpc::remote_capnp::ChangeSeq::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(1).get_list(::rpc::remote_capnp::ChangeSeq::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_changeseqs(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_changes(&self) -> StructList::Reader<'a,::rpc::remote_capnp::Change::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(2).get_list(::rpc::remote_capnp::Change::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_changes(&self) -> bool {
        !self.reader.get_pointer_field(2).is_null()
      }
      #[inline]
      pub fn get_patches(&self) -> StructList::Reader<'a,::rpc::remote_capnp::Patch::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(3).get_list(::rpc::remote_capnp::Patch::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_patches(&self) -> bool {
        !self.reader.get_pointer_field(3).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_commit(&self) -> ::rpc::remote_capnp::Commit::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::rpc::remote_capnp::Commit::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_commit(&self, value : ::rpc::remote_capnp::Commit::Reader) {
        self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
      }
      #[inline]
      pub fn init_commit(&self, ) -> ::rpc::remote_capnp::Commit::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::rpc::remote_capnp::Commit::STRUCT_SIZE))
      }
      pub fn has_commit(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_changeseqs(&self) -> StructList::Builder<'a,::rpc::remote_capnp::ChangeSeq::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(1).get_struct_list(::rpc::remote_capnp::ChangeSeq::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_changeseqs(&self, value : StructList::Reader<'a,::rpc::remote_capnp::ChangeSeq::Reader<'a>>) {
        self.builder.get_pointer_field(1).set_list(&value.reader)
      }
      #[inline]
      pub fn init_changeseqs(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::ChangeSeq::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::ChangeSeq::Builder<'a>>::new(
          self.builder.get_pointer_field(1).init_struct_list(size, ::rpc::remote_capnp::ChangeSeq::STRUCT_SIZE))
      }
      pub fn has_changeseqs(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_changes(&self) -> StructList::Builder<'a,::rpc::remote_capnp::Change::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(2).get_struct_list(::rpc::remote_capnp::Change::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_changes(&self, value : StructList::Reader<'a,::rpc::remote_capnp::Change::Reader<'a>>) {
        self.builder.get_pointer_field(2).set_list(&value.reader)
      }
      #[inline]
      pub fn init_changes(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::Change::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::Change::Builder<'a>>::new(
          self.builder.get_pointer_field(2).init_struct_list(size, ::rpc::remote_capnp::Change::STRUCT_SIZE))
      }
      pub fn has_changes(&self) -> bool {
        !self.builder.get_pointer_field(2).is_null()
      }
      #[inline]
      pub fn get_patches(&self) -> StructList::Builder<'a,::rpc::remote_capnp::Patch::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(3).get_struct_list(::rpc::remote_capnp::Patch::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_patches(&self, value : StructList::Reader<'a,::rpc::remote_capnp::Patch::Reader<'a>>) {
        self.builder.get_pointer_field(3).set_list(&value.reader)
      }
      #[inline]
      pub fn init_patches(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::Patch::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::Patch::Builder<'a>>::new(
          self.builder.get_pointer_field(3).init_struct_list(size, ::rpc::remote_capnp::Patch::STRUCT_SIZE))
      }
      pub fn has_patches(&self) -> bool {
        !self.builder.get_pointer_field(3).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
      pub fn get_commit(&self) -> ::rpc::remote_capnp::Commit::Pipeline {
        FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
      }
    }
  }

  pub mod PushResults {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 0, preferred_list_encoding : layout::Void};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod TagParams {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 3, preferred_list_encoding : layout::InlineComposite};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_key(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      pub fn has_key(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_value(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      pub fn has_value(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
        FromStructReader::new(self.reader.get_pointer_field(2).get_struct( std::ptr::null()))
      }
      pub fn has_id(&self) -> bool {
        !self.reader.get_pointer_field(2).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_key(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_key(&self, value : Text::Reader) {
        self.builder.get_pointer_field(0).set_text(value);
      }
      #[inline]
      pub fn init_key(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).init_text(size)
      }
      pub fn has_key(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_value(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_value(&self, value : Text::Reader) {
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_value(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_value(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(2).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
        self.builder.get_pointer_field(2).set_struct(&value.struct_reader())
      }
      #[inline]
      pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(2).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
      }
      pub fn has_id(&self) -> bool {
        !self.builder.get_pointer_field(2).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
        FromTypelessPipeline::new(self._typeless.get_pointer_field(2))
      }
    }
  }

  pub mod TagResults {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 1, pointers : 0, preferred_list_encoding : layout::Bit};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_result(&self) -> bool {
        self.reader.get_bool_field(0)
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_result(&self) -> bool {
        self.builder.get_bool_field(0)
      }
      #[inline]
      pub fn set_result(&self, value : bool) {
        self.builder.set_bool_field(0, value);
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod UntagParams {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 3, preferred_list_encoding : layout::InlineComposite};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_key(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      pub fn has_key(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_value(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      pub fn has_value(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Reader<'a> {
        FromStructReader::new(self.reader.get_pointer_field(2).get_struct( std::ptr::null()))
      }
      pub fn has_id(&self) -> bool {
        !self.reader.get_pointer_field(2).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_key(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_key(&self, value : Text::Reader) {
        self.builder.get_pointer_field(0).set_text(value);
      }
      #[inline]
      pub fn init_key(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).init_text(size)
      }
      pub fn has_key(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_value(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_value(&self, value : Text::Reader) {
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_value(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_value(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(2).get_struct(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_id(&self, value : ::rpc::remote_capnp::ID::Reader) {
        self.builder.get_pointer_field(2).set_struct(&value.struct_reader())
      }
      #[inline]
      pub fn init_id(&self, ) -> ::rpc::remote_capnp::ID::Builder<'a> {
        FromStructBuilder::new(self.builder.get_pointer_field(2).init_struct(::rpc::remote_capnp::ID::STRUCT_SIZE))
      }
      pub fn has_id(&self) -> bool {
        !self.builder.get_pointer_field(2).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
      pub fn get_id(&self) -> ::rpc::remote_capnp::ID::Pipeline {
        FromTypelessPipeline::new(self._typeless.get_pointer_field(2))
      }
    }
  }

  pub mod UntagResults {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 1, pointers : 0, preferred_list_encoding : layout::Bit};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_result(&self) -> bool {
        self.reader.get_bool_field(0)
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_result(&self) -> bool {
        self.builder.get_bool_field(0)
      }
      #[inline]
      pub fn set_result(&self, value : bool) {
        self.builder.set_bool_field(0, value);
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod TaggedParams {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 2, preferred_list_encoding : layout::InlineComposite};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_key(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      pub fn has_key(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_value(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      pub fn has_value(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_key(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_key(&self, value : Text::Reader) {
        self.builder.get_pointer_field(0).set_text(value);
      }
      #[inline]
      pub fn init_key(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).init_text(size)
      }
      pub fn has_key(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_value(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_value(&self, value : Text::Reader) {
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_value(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_value(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod TaggedResults {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 0, pointers : 1, preferred_list_encoding : layout::Pointer};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_objects(&self) -> StructList::Reader<'a,::rpc::remote_capnp::ID::Reader<'a>> {
        StructList::Reader::new(self.reader.get_pointer_field(0).get_list(::rpc::remote_capnp::ID::STRUCT_SIZE.preferred_list_encoding, std::ptr::null()))
      }
      pub fn has_objects(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_objects(&self) -> StructList::Builder<'a,::rpc::remote_capnp::ID::Builder<'a>> {
        StructList::Builder::new(self.builder.get_pointer_field(0).get_struct_list(::rpc::remote_capnp::ID::STRUCT_SIZE, std::ptr::null()))
      }
      #[inline]
      pub fn set_objects(&self, value : StructList::Reader<'a,::rpc::remote_capnp::ID::Reader<'a>>) {
        self.builder.get_pointer_field(0).set_list(&value.reader)
      }
      #[inline]
      pub fn init_objects(&self, size : uint) -> StructList::Builder<'a,::rpc::remote_capnp::ID::Builder<'a>> {
        StructList::Builder::<'a, ::rpc::remote_capnp::ID::Builder<'a>>::new(
          self.builder.get_pointer_field(0).init_struct_list(size, ::rpc::remote_capnp::ID::STRUCT_SIZE))
      }
      pub fn has_objects(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }
}
