// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

// TODO: Hand edited! Figure out a better solution for objecthash support

use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Witness {
    // message fields
    signatures: ::protobuf::RepeatedField<super::signature::Signature>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Witness {}

impl Witness {
    pub fn new() -> Witness {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Witness {
        static mut instance: ::protobuf::lazy::Lazy<Witness> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Witness,
        };
        unsafe { instance.get(Witness::new) }
    }

    // repeated .ithos.Signature signatures = 1;

    pub fn clear_signatures(&mut self) {
        self.signatures.clear();
    }

    // Param is passed by value, moved
    pub fn set_signatures(&mut self, v: ::protobuf::RepeatedField<super::signature::Signature>) {
        self.signatures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signatures(&mut self)
                          -> &mut ::protobuf::RepeatedField<super::signature::Signature> {
        &mut self.signatures
    }

    // Take field
    pub fn take_signatures(&mut self) -> ::protobuf::RepeatedField<super::signature::Signature> {
        ::std::mem::replace(&mut self.signatures, ::protobuf::RepeatedField::new())
    }

    pub fn get_signatures(&self) -> &[super::signature::Signature] {
        &self.signatures
    }

    fn get_signatures_for_reflect(&self)
                                  -> &::protobuf::RepeatedField<super::signature::Signature> {
        &self.signatures
    }

    fn mut_signatures_for_reflect
        (&mut self)
         -> &mut ::protobuf::RepeatedField<super::signature::Signature> {
        &mut self.signatures
    }
}

impl ::protobuf::Message for Witness {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type,
                                                               is,
                                                               &mut self.signatures)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.signatures {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        for v in &self.signatures {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Witness {
    fn new() -> Witness {
        Witness::new()
    }

    fn descriptor_static(_: ::std::option::Option<Witness>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::signature::Signature>>(
                    "signatures",
                    Witness::get_signatures_for_reflect,
                    Witness::mut_signatures_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Witness>(
                    "Witness",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Witness {
    fn clear(&mut self) {
        self.clear_signatures();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Witness {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Witness {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x0d, 0x77, 0x69, 0x74, 0x6e, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
      0x12, 0x05, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x1a, 0x0f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74,
      0x75, 0x72, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3b, 0x0a, 0x07, 0x57, 0x69,
      0x74, 0x6e, 0x65, 0x73, 0x73, 0x12, 0x30, 0x0a, 0x0a, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74,
      0x75, 0x72, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x69, 0x74,
      0x68, 0x6f, 0x73, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x52, 0x0a,
      0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x4a, 0xc8, 0x01, 0x0a, 0x06,
      0x12, 0x04, 0x00, 0x00, 0x09, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
      0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x09, 0x0a, 0x02,
      0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x18, 0x0a, 0x4e, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
      0x07, 0x00, 0x09, 0x01, 0x1a, 0x42, 0x20, 0x57, 0x69, 0x74, 0x6e, 0x65, 0x73, 0x73, 0x20,
      0x28, 0x69, 0x2e, 0x65, 0x2e, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65,
      0x29, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x61, 0x69, 0x74,
      0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69,
      0x63, 0x75, 0x6c, 0x61, 0x72, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x0a, 0x0a, 0x0a,
      0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
      0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
      0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06,
      0x12, 0x03, 0x08, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
      0x03, 0x08, 0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
      0x08, 0x24, 0x25, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}

// TODO: Hand edited! Figure out a better solution for objecthash support
impl ObjectHash for Witness {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(hasher, "signatures" => &Vec::from(self.get_signatures()))
    }
}
