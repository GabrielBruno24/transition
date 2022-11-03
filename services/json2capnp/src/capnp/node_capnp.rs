// @generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: node.capnp


pub mod node {
  #[derive(Copy, Clone)]
  pub struct Owned(());
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<'_,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_uuid(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
    }
    pub fn has_uuid(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_id(self) -> u32 {
      self.reader.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn get_internal_id(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
    }
    pub fn has_internal_id(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_code(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::core::option::Option::None)
    }
    pub fn has_code(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(3), ::core::option::Option::None)
    }
    pub fn has_name(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_latitude(self) -> i32 {
      self.reader.get_data_field::<i32>(1)
    }
    #[inline]
    pub fn get_longitude(self) -> i32 {
      self.reader.get_data_field::<i32>(2)
    }
    #[inline]
    pub fn get_station_uuid(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(4), ::core::option::Option::None)
    }
    pub fn has_station_uuid(&self) -> bool {
      !self.reader.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_color(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(5), ::core::option::Option::None)
    }
    pub fn has_color(&self) -> bool {
      !self.reader.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_description(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(6), ::core::option::Option::None)
    }
    pub fn has_description(&self) -> bool {
      !self.reader.get_pointer_field(6).is_null()
    }
    #[inline]
    pub fn get_data(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(7), ::core::option::Option::None)
    }
    pub fn has_data(&self) -> bool {
      !self.reader.get_pointer_field(7).is_null()
    }
    #[inline]
    pub fn get_is_enabled(self) -> i8 {
      self.reader.get_data_field::<i8>(12)
    }
    #[inline]
    pub fn get_routing_radius_meters(self) -> i16 {
      self.reader.get_data_field::<i16>(7)
    }
    #[inline]
    pub fn get_default_dwell_time_seconds(self) -> i16 {
      self.reader.get_data_field::<i16>(8)
    }
    #[inline]
    pub fn get_transferable_nodes_uuids(self) -> ::capnp::Result<::capnp::text_list::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(8), ::core::option::Option::None)
    }
    pub fn has_transferable_nodes_uuids(&self) -> bool {
      !self.reader.get_pointer_field(8).is_null()
    }
    #[inline]
    pub fn get_transferable_nodes_travel_times(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,i16>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(9), ::core::option::Option::None)
    }
    pub fn has_transferable_nodes_travel_times(&self) -> bool {
      !self.reader.get_pointer_field(9).is_null()
    }
    #[inline]
    pub fn get_transferable_nodes_distances(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,i16>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(10), ::core::option::Option::None)
    }
    pub fn has_transferable_nodes_distances(&self) -> bool {
      !self.reader.get_pointer_field(10).is_null()
    }
    #[inline]
    pub fn get_is_frozen(self) -> i8 {
      self.reader.get_data_field::<i8>(13)
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn reborrow(&mut self) -> Builder<'_,> {
      Builder { .. *self }
    }
    pub fn reborrow_as_reader(&self) -> Reader<'_,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn get_uuid(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_uuid(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_uuid(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    pub fn has_uuid(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_id(self) -> u32 {
      self.builder.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn set_id(&mut self, value: u32)  {
      self.builder.set_data_field::<u32>(0, value);
    }
    #[inline]
    pub fn get_internal_id(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_internal_id(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(1).set_text(value);
    }
    #[inline]
    pub fn init_internal_id(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(1).init_text(size)
    }
    pub fn has_internal_id(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_code(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_code(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(2).set_text(value);
    }
    #[inline]
    pub fn init_code(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(2).init_text(size)
    }
    pub fn has_code(&self) -> bool {
      !self.builder.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_name(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(3).set_text(value);
    }
    #[inline]
    pub fn init_name(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(3).init_text(size)
    }
    pub fn has_name(&self) -> bool {
      !self.builder.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_latitude(self) -> i32 {
      self.builder.get_data_field::<i32>(1)
    }
    #[inline]
    pub fn set_latitude(&mut self, value: i32)  {
      self.builder.set_data_field::<i32>(1, value);
    }
    #[inline]
    pub fn get_longitude(self) -> i32 {
      self.builder.get_data_field::<i32>(2)
    }
    #[inline]
    pub fn set_longitude(&mut self, value: i32)  {
      self.builder.set_data_field::<i32>(2, value);
    }
    #[inline]
    pub fn get_station_uuid(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_station_uuid(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(4).set_text(value);
    }
    #[inline]
    pub fn init_station_uuid(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(4).init_text(size)
    }
    pub fn has_station_uuid(&self) -> bool {
      !self.builder.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_color(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_color(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(5).set_text(value);
    }
    #[inline]
    pub fn init_color(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(5).init_text(size)
    }
    pub fn has_color(&self) -> bool {
      !self.builder.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_description(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_description(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(6).set_text(value);
    }
    #[inline]
    pub fn init_description(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(6).init_text(size)
    }
    pub fn has_description(&self) -> bool {
      !self.builder.get_pointer_field(6).is_null()
    }
    #[inline]
    pub fn get_data(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(7), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_data(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(7).set_text(value);
    }
    #[inline]
    pub fn init_data(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(7).init_text(size)
    }
    pub fn has_data(&self) -> bool {
      !self.builder.get_pointer_field(7).is_null()
    }
    #[inline]
    pub fn get_is_enabled(self) -> i8 {
      self.builder.get_data_field::<i8>(12)
    }
    #[inline]
    pub fn set_is_enabled(&mut self, value: i8)  {
      self.builder.set_data_field::<i8>(12, value);
    }
    #[inline]
    pub fn get_routing_radius_meters(self) -> i16 {
      self.builder.get_data_field::<i16>(7)
    }
    #[inline]
    pub fn set_routing_radius_meters(&mut self, value: i16)  {
      self.builder.set_data_field::<i16>(7, value);
    }
    #[inline]
    pub fn get_default_dwell_time_seconds(self) -> i16 {
      self.builder.get_data_field::<i16>(8)
    }
    #[inline]
    pub fn set_default_dwell_time_seconds(&mut self, value: i16)  {
      self.builder.set_data_field::<i16>(8, value);
    }
    #[inline]
    pub fn get_transferable_nodes_uuids(self) -> ::capnp::Result<::capnp::text_list::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(8), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_transferable_nodes_uuids(&mut self, value: ::capnp::text_list::Reader<'a>) -> ::capnp::Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(8), value, false)
    }
    #[inline]
    pub fn init_transferable_nodes_uuids(self, size: u32) -> ::capnp::text_list::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(8), size)
    }
    pub fn has_transferable_nodes_uuids(&self) -> bool {
      !self.builder.get_pointer_field(8).is_null()
    }
    #[inline]
    pub fn get_transferable_nodes_travel_times(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,i16>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(9), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_transferable_nodes_travel_times(&mut self, value: ::capnp::primitive_list::Reader<'a,i16>) -> ::capnp::Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(9), value, false)
    }
    #[inline]
    pub fn init_transferable_nodes_travel_times(self, size: u32) -> ::capnp::primitive_list::Builder<'a,i16> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(9), size)
    }
    pub fn has_transferable_nodes_travel_times(&self) -> bool {
      !self.builder.get_pointer_field(9).is_null()
    }
    #[inline]
    pub fn get_transferable_nodes_distances(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,i16>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(10), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_transferable_nodes_distances(&mut self, value: ::capnp::primitive_list::Reader<'a,i16>) -> ::capnp::Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(10), value, false)
    }
    #[inline]
    pub fn init_transferable_nodes_distances(self, size: u32) -> ::capnp::primitive_list::Builder<'a,i16> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(10), size)
    }
    pub fn has_transferable_nodes_distances(&self) -> bool {
      !self.builder.get_pointer_field(10).is_null()
    }
    #[inline]
    pub fn get_is_frozen(self) -> i8 {
      self.builder.get_data_field::<i8>(13)
    }
    #[inline]
    pub fn set_is_frozen(&mut self, value: i8)  {
      self.builder.set_data_field::<i8>(13, value);
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 3, pointers: 11 };
    pub const TYPE_ID: u64 = 0xcfb0_309e_1293_c2e3;
  }
}
