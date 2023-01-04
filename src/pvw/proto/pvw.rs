// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `pvw.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PvwSecretKey)
pub struct PvwSecretKey {
    // message fields
    // @@protoc_insertion_point(field:PvwSecretKey.key)
    pub key: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:PvwSecretKey.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PvwSecretKey {
    fn default() -> &'a PvwSecretKey {
        <PvwSecretKey as ::protobuf::Message>::default_instance()
    }
}

impl PvwSecretKey {
    pub fn new() -> PvwSecretKey {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "key",
            |m: &PvwSecretKey| { &m.key },
            |m: &mut PvwSecretKey| { &mut m.key },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PvwSecretKey>(
            "PvwSecretKey",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PvwSecretKey {
    const NAME: &'static str = "PvwSecretKey";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.key = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PvwSecretKey {
        PvwSecretKey::new()
    }

    fn clear(&mut self) {
        self.key.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PvwSecretKey {
        static instance: PvwSecretKey = PvwSecretKey {
            key: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PvwSecretKey {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PvwSecretKey").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PvwSecretKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PvwSecretKey {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PvwCiphertext)
pub struct PvwCiphertext {
    // message fields
    // @@protoc_insertion_point(field:PvwCiphertext.a)
    pub a: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:PvwCiphertext.b)
    pub b: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:PvwCiphertext.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PvwCiphertext {
    fn default() -> &'a PvwCiphertext {
        <PvwCiphertext as ::protobuf::Message>::default_instance()
    }
}

impl PvwCiphertext {
    pub fn new() -> PvwCiphertext {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "a",
            |m: &PvwCiphertext| { &m.a },
            |m: &mut PvwCiphertext| { &mut m.a },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "b",
            |m: &PvwCiphertext| { &m.b },
            |m: &mut PvwCiphertext| { &mut m.b },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PvwCiphertext>(
            "PvwCiphertext",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PvwCiphertext {
    const NAME: &'static str = "PvwCiphertext";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.a = is.read_bytes()?;
                },
                18 => {
                    self.b = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.a.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.a);
        }
        if !self.b.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.b);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.a.is_empty() {
            os.write_bytes(1, &self.a)?;
        }
        if !self.b.is_empty() {
            os.write_bytes(2, &self.b)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PvwCiphertext {
        PvwCiphertext::new()
    }

    fn clear(&mut self) {
        self.a.clear();
        self.b.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PvwCiphertext {
        static instance: PvwCiphertext = PvwCiphertext {
            a: ::std::vec::Vec::new(),
            b: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PvwCiphertext {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PvwCiphertext").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PvwCiphertext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PvwCiphertext {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PvwPublicKey)
pub struct PvwPublicKey {
    // message fields
    // @@protoc_insertion_point(field:PvwPublicKey.a)
    pub a: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:PvwPublicKey.b)
    pub b: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:PvwPublicKey.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PvwPublicKey {
    fn default() -> &'a PvwPublicKey {
        <PvwPublicKey as ::protobuf::Message>::default_instance()
    }
}

impl PvwPublicKey {
    pub fn new() -> PvwPublicKey {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "a",
            |m: &PvwPublicKey| { &m.a },
            |m: &mut PvwPublicKey| { &mut m.a },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "b",
            |m: &PvwPublicKey| { &m.b },
            |m: &mut PvwPublicKey| { &mut m.b },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PvwPublicKey>(
            "PvwPublicKey",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PvwPublicKey {
    const NAME: &'static str = "PvwPublicKey";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.a = is.read_bytes()?;
                },
                18 => {
                    self.b = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.a.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.a);
        }
        if !self.b.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.b);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.a.is_empty() {
            os.write_bytes(1, &self.a)?;
        }
        if !self.b.is_empty() {
            os.write_bytes(2, &self.b)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PvwPublicKey {
        PvwPublicKey::new()
    }

    fn clear(&mut self) {
        self.a.clear();
        self.b.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PvwPublicKey {
        static instance: PvwPublicKey = PvwPublicKey {
            a: ::std::vec::Vec::new(),
            b: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PvwPublicKey {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PvwPublicKey").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PvwPublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PvwPublicKey {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tpvw.proto\"\x20\n\x0cPvwSecretKey\x12\x10\n\x03key\x18\x01\x20\x01(\
    \x0cR\x03key\"+\n\rPvwCiphertext\x12\x0c\n\x01a\x18\x01\x20\x01(\x0cR\
    \x01a\x12\x0c\n\x01b\x18\x02\x20\x01(\x0cR\x01b\"*\n\x0cPvwPublicKey\x12\
    \x0c\n\x01a\x18\x01\x20\x01(\x0cR\x01a\x12\x0c\n\x01b\x18\x02\x20\x01(\
    \x0cR\x01bJ\xed\x02\n\x06\x12\x04\0\0\x0e\x01\n\x08\n\x01\x0c\x12\x03\0\
    \0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x04\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x02\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x12\n\x0c\n\x05\x04\
    \0\x02\0\x05\x12\x03\x03\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\n\
    \r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x10\x11\n\n\n\x02\x04\x01\x12\
    \x04\x06\0\t\x01\n\n\n\x03\x04\x01\x01\x12\x03\x06\x08\x15\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x07\x04\x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x07\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x07\n\x0b\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x07\x0e\x0f\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03\x08\x04\x10\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x08\x04\t\n\x0c\
    \n\x05\x04\x01\x02\x01\x01\x12\x03\x08\n\x0b\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x08\x0e\x0f\n\n\n\x02\x04\x02\x12\x04\x0b\0\x0e\x01\n\n\n\
    \x03\x04\x02\x01\x12\x03\x0b\x08\x14\n\x0b\n\x04\x04\x02\x02\0\x12\x03\
    \x0c\x04\x10\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x0c\x04\t\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\x0c\n\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03\x0c\x0e\x0f\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\r\x04\x10\n\x0c\n\
    \x05\x04\x02\x02\x01\x05\x12\x03\r\x04\t\n\x0c\n\x05\x04\x02\x02\x01\x01\
    \x12\x03\r\n\x0b\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\r\x0e\x0fb\x06p\
    roto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(PvwSecretKey::generated_message_descriptor_data());
            messages.push(PvwCiphertext::generated_message_descriptor_data());
            messages.push(PvwPublicKey::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
