// This file is generated by rust-protobuf 3.0.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const expose_oneof_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const repeated_field_vec_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17020, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };

    pub const serde_derive_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const repeated_field_vec: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17020, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };

    pub const serde_derive: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_fields_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const repeated_field_vec_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17020, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:J\n\x11expose_fields_all\x18\
    \xeb\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0fexpose\
    FieldsAll:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x14generateAccessorsAll:N\n\x13generat\
    e_getter_all\x18\xed\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOp\
    tionsR\x11generateGetterAll:b\n\x1ecarllerche_bytes_for_bytes_all\x18\
    \xf3\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1acarlle\
    rcheBytesForBytesAll:d\n\x1fcarllerche_bytes_for_string_all\x18\xf4\x84\
    \x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1bcarllercheByte\
    sForStringAll:S\n\x16repeated_field_vec_all\x18\xfc\x84\x01\x20\x01(\x08\
    \x12\x1c.google.protobuf.FileOptionsR\x13repeatedFieldVecAll:`\n\x1dsing\
    ular_field_option_box_all\x18\x80\x85\x01\x20\x01(\x08\x12\x1c.google.pr\
    otobuf.FileOptionsR\x19singularFieldOptionBoxAll:Y\n\x19singular_field_o\
    ption_all\x18\x81\x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptio\
    nsR\x16singularFieldOptionAll:H\n\x10serde_derive_all\x18\x86\x85\x01\
    \x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eserdeDeriveAll:O\n\
    \x14serde_derive_cfg_all\x18\x87\x85\x01\x20\x01(\t\x12\x1c.google.proto\
    buf.FileOptionsR\x11serdeDeriveCfgAll:D\n\x0cexpose_oneof\x18\xe9\x84\
    \x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bexposeOneof\
    :F\n\rexpose_fields\x18\xeb\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf\
    .MessageOptionsR\x0cexposeFields:P\n\x12generate_accessors\x18\xec\x84\
    \x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x11generateAcc\
    essors:J\n\x0fgenerate_getter\x18\xed\x84\x01\x20\x01(\x08\x12\x1f.googl\
    e.protobuf.MessageOptionsR\x0egenerateGetter:^\n\x1acarllerche_bytes_for\
    _bytes\x18\xf3\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptio\
    nsR\x17carllercheBytesForBytes:`\n\x1bcarllerche_bytes_for_string\x18\
    \xf4\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x18car\
    llercheBytesForString:O\n\x12repeated_field_vec\x18\xfc\x84\x01\x20\x01(\
    \x08\x12\x1f.google.protobuf.MessageOptionsR\x10repeatedFieldVec:\\\n\
    \x19singular_field_option_box\x18\x80\x85\x01\x20\x01(\x08\x12\x1f.googl\
    e.protobuf.MessageOptionsR\x16singularFieldOptionBox:U\n\x15singular_fie\
    ld_option\x18\x81\x85\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOp\
    tionsR\x13singularFieldOption:D\n\x0cserde_derive\x18\x86\x85\x01\x20\
    \x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bserdeDerive:K\n\x10\
    serde_derive_cfg\x18\x87\x85\x01\x20\x01(\t\x12\x1f.google.protobuf.Mess\
    ageOptionsR\x0eserdeDeriveCfg:O\n\x13expose_fields_field\x18\xeb\x84\x01\
    \x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x11exposeFieldsField\
    :Y\n\x18generate_accessors_field\x18\xec\x84\x01\x20\x01(\x08\x12\x1d.go\
    ogle.protobuf.FieldOptionsR\x16generateAccessorsField:S\n\x15generate_ge\
    tter_field\x18\xed\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOpt\
    ionsR\x13generateGetterField:g\n\x20carllerche_bytes_for_bytes_field\x18\
    \xf3\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1ccarll\
    ercheBytesForBytesField:i\n!carllerche_bytes_for_string_field\x18\xf4\
    \x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1dcarllerch\
    eBytesForStringField:X\n\x18repeated_field_vec_field\x18\xfc\x84\x01\x20\
    \x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x15repeatedFieldVecField\
    :e\n\x1fsingular_field_option_box_field\x18\x80\x85\x01\x20\x01(\x08\x12\
    \x1d.google.protobuf.FieldOptionsR\x1bsingularFieldOptionBoxField:^\n\
    \x1bsingular_field_option_field\x18\x81\x85\x01\x20\x01(\x08\x12\x1d.goo\
    gle.protobuf.FieldOptionsR\x18singularFieldOptionFieldJ\x88\"\n\x06\x12\
    \x04\0\0S\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\
    \x02\x07)\n\xe5\x01\n\x01\x02\x12\x03\n\x08\x112^\x20see\x20https://gith\
    ub.com/gogo/protobuf/blob/master/gogoproto/gogo.proto\n\x20for\x20the\
    \x20original\x20idea\n2{\x20Generated\x20files\x20can\x20be\x20customize\
    d\x20using\x20this\x20proto\n\x20or\x20using\x20`Customize`\x20struct\
    \x20when\x20codegen\x20is\x20invoked\x20programmatically.\n\n\t\n\x01\
    \x07\x12\x04\x0c\0%\x01\n7\n\x02\x07\0\x12\x03\x0e\x04+\x1a,\x20When\x20\
    true,\x20oneof\x20field\x20is\x20generated\x20public\n\n\n\n\x03\x07\0\
    \x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\0\x04\x12\x03\x0e\x04\x0c\n\n\n\x03\
    \x07\0\x05\x12\x03\x0e\r\x11\n\n\n\x03\x07\0\x01\x12\x03\x0e\x12\"\n\n\n\
    \x03\x07\0\x03\x12\x03\x0e%*\nI\n\x02\x07\x01\x12\x03\x10\x04,\x1a>\x20W\
    hen\x20true\x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessor\
    s\x20generated\n\n\n\n\x03\x07\x01\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\
    \x01\x04\x12\x03\x10\x04\x0c\n\n\n\x03\x07\x01\x05\x12\x03\x10\r\x11\n\n\
    \n\x03\x07\x01\x01\x12\x03\x10\x12#\n\n\n\x03\x07\x01\x03\x12\x03\x10&+\
    \nP\n\x02\x07\x02\x12\x03\x12\x041\x1aE\x20When\x20false,\x20`get_`,\x20\
    `set_`,\x20`mut_`\x20etc.\x20accessors\x20are\x20not\x20generated\n\n\n\
    \n\x03\x07\x02\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x02\x04\x12\x03\x12\
    \x04\x0c\n\n\n\x03\x07\x02\x05\x12\x03\x12\r\x11\n\n\n\x03\x07\x02\x01\
    \x12\x03\x12\x12(\n\n\n\x03\x07\x02\x03\x12\x03\x12+0\nL\n\x02\x07\x03\
    \x12\x03\x14\x04.\x1aA\x20When\x20false,\x20`get_`\x20is\x20not\x20gener\
    ated\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\n\x03\x07\x03\
    \x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x03\x04\x12\x03\x14\x04\x0c\n\n\n\
    \x03\x07\x03\x05\x12\x03\x14\r\x11\n\n\n\x03\x07\x03\x01\x12\x03\x14\x12\
    %\n\n\n\x03\x07\x03\x03\x12\x03\x14(-\n2\n\x02\x07\x04\x12\x03\x16\x049\
    \x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\
    \x07\x04\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x04\x04\x12\x03\x16\x04\x0c\
    \n\n\n\x03\x07\x04\x05\x12\x03\x16\r\x11\n\n\n\x03\x07\x04\x01\x12\x03\
    \x16\x120\n\n\n\x03\x07\x04\x03\x12\x03\x1638\n3\n\x02\x07\x05\x12\x03\
    \x18\x04:\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\
    \n\n\x03\x07\x05\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x05\x04\x12\x03\x18\
    \x04\x0c\n\n\n\x03\x07\x05\x05\x12\x03\x18\r\x11\n\n\n\x03\x07\x05\x01\
    \x12\x03\x18\x121\n\n\n\x03\x07\x05\x03\x12\x03\x1849\n=\n\x02\x07\x06\
    \x12\x03\x1a\x041\x1a2\x20Use\x20`std::Vec`\x20to\x20store\x20repeated\
    \x20messages\x20fields\n\n\n\n\x03\x07\x06\x02\x12\x03\x0c\x07\"\n\n\n\
    \x03\x07\x06\x04\x12\x03\x1a\x04\x0c\n\n\n\x03\x07\x06\x05\x12\x03\x1a\r\
    \x11\n\n\n\x03\x07\x06\x01\x12\x03\x1a\x12(\n\n\n\x03\x07\x06\x03\x12\
    \x03\x1a+0\nM\n\x02\x07\x07\x12\x03\x1c\x048\x1aB\x20Use\x20`std::Option\
    <std::Box<T>>`\x20to\x20store\x20singular\x20messages\x20fields\n\n\n\n\
    \x03\x07\x07\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x07\x04\x12\x03\x1c\x04\
    \x0c\n\n\n\x03\x07\x07\x05\x12\x03\x1c\r\x11\n\n\n\x03\x07\x07\x01\x12\
    \x03\x1c\x12/\n\n\n\x03\x07\x07\x03\x12\x03\x1c27\n\x93\x01\n\x02\x07\
    \x08\x12\x03\x1f\x044\x1a\x87\x01\x20Use\x20`std::Option<T>`\x20to\x20st\
    ore\x20singular\x20messages\x20fields.\n\x20Note,\x20it's\x20not\x20poss\
    ible\x20to\x20have\x20recursive\x20messages\x20with\x20this\x20option\
    \x20enabled.\n\n\n\n\x03\x07\x08\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x08\
    \x04\x12\x03\x1f\x04\x0c\n\n\n\x03\x07\x08\x05\x12\x03\x1f\r\x11\n\n\n\
    \x03\x07\x08\x01\x12\x03\x1f\x12+\n\n\n\x03\x07\x08\x03\x12\x03\x1f.3\nJ\
    \n\x02\x07\t\x12\x03\"\x04+\x1a?\x20Use\x20`serde_derive`\x20to\x20imple\
    ment\x20`Serialize`\x20and\x20`Deserialize`\n\n\n\n\x03\x07\t\x02\x12\
    \x03\x0c\x07\"\n\n\n\x03\x07\t\x04\x12\x03\"\x04\x0c\n\n\n\x03\x07\t\x05\
    \x12\x03\"\r\x11\n\n\n\x03\x07\t\x01\x12\x03\"\x12\"\n\n\n\x03\x07\t\x03\
    \x12\x03\"%*\n3\n\x02\x07\n\x12\x03$\x041\x1a(\x20Guard\x20serde\x20anno\
    tations\x20with\x20cfg\x20attr.\n\n\n\n\x03\x07\n\x02\x12\x03\x0c\x07\"\
    \n\n\n\x03\x07\n\x04\x12\x03$\x04\x0c\n\n\n\x03\x07\n\x05\x12\x03$\r\x13\
    \n\n\n\x03\x07\n\x01\x12\x03$\x14(\n\n\n\x03\x07\n\x03\x12\x03$+0\n\t\n\
    \x01\x07\x12\x04'\0?\x01\n7\n\x02\x07\x0b\x12\x03)\x04'\x1a,\x20When\x20\
    true,\x20oneof\x20field\x20is\x20generated\x20public\n\n\n\n\x03\x07\x0b\
    \x02\x12\x03'\x07%\n\n\n\x03\x07\x0b\x04\x12\x03)\x04\x0c\n\n\n\x03\x07\
    \x0b\x05\x12\x03)\r\x11\n\n\n\x03\x07\x0b\x01\x12\x03)\x12\x1e\n\n\n\x03\
    \x07\x0b\x03\x12\x03)!&\nI\n\x02\x07\x0c\x12\x03+\x04(\x1a>\x20When\x20t\
    rue\x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\x20gen\
    erated\n\n\n\n\x03\x07\x0c\x02\x12\x03'\x07%\n\n\n\x03\x07\x0c\x04\x12\
    \x03+\x04\x0c\n\n\n\x03\x07\x0c\x05\x12\x03+\r\x11\n\n\n\x03\x07\x0c\x01\
    \x12\x03+\x12\x1f\n\n\n\x03\x07\x0c\x03\x12\x03+\"'\nP\n\x02\x07\r\x12\
    \x03-\x04-\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc\
    .\x20accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\r\x02\x12\x03'\
    \x07%\n\n\n\x03\x07\r\x04\x12\x03-\x04\x0c\n\n\n\x03\x07\r\x05\x12\x03-\
    \r\x11\n\n\n\x03\x07\r\x01\x12\x03-\x12$\n\n\n\x03\x07\r\x03\x12\x03-',\
    \nL\n\x02\x07\x0e\x12\x03/\x04*\x1aA\x20When\x20false,\x20`get_`\x20is\
    \x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\
    \n\x03\x07\x0e\x02\x12\x03'\x07%\n\n\n\x03\x07\x0e\x04\x12\x03/\x04\x0c\
    \n\n\n\x03\x07\x0e\x05\x12\x03/\r\x11\n\n\n\x03\x07\x0e\x01\x12\x03/\x12\
    !\n\n\n\x03\x07\x0e\x03\x12\x03/$)\n2\n\x02\x07\x0f\x12\x031\x045\x1a'\
    \x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\
    \x0f\x02\x12\x03'\x07%\n\n\n\x03\x07\x0f\x04\x12\x031\x04\x0c\n\n\n\x03\
    \x07\x0f\x05\x12\x031\r\x11\n\n\n\x03\x07\x0f\x01\x12\x031\x12,\n\n\n\
    \x03\x07\x0f\x03\x12\x031/4\n3\n\x02\x07\x10\x12\x033\x046\x1a(\x20Use\
    \x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x10\x02\
    \x12\x03'\x07%\n\n\n\x03\x07\x10\x04\x12\x033\x04\x0c\n\n\n\x03\x07\x10\
    \x05\x12\x033\r\x11\n\n\n\x03\x07\x10\x01\x12\x033\x12-\n\n\n\x03\x07\
    \x10\x03\x12\x03305\n<\n\x02\x07\x11\x12\x035\x04-\x1a1\x20Use\x20`std::\
    Vec`\x20to\x20store\x20repeated\x20messages\x20field\n\n\n\n\x03\x07\x11\
    \x02\x12\x03'\x07%\n\n\n\x03\x07\x11\x04\x12\x035\x04\x0c\n\n\n\x03\x07\
    \x11\x05\x12\x035\r\x11\n\n\n\x03\x07\x11\x01\x12\x035\x12$\n\n\n\x03\
    \x07\x11\x03\x12\x035',\nM\n\x02\x07\x12\x12\x037\x044\x1aB\x20Use\x20`s\
    td::Option<std::Box<T>>`\x20to\x20store\x20singular\x20messages\x20field\
    s\n\n\n\n\x03\x07\x12\x02\x12\x03'\x07%\n\n\n\x03\x07\x12\x04\x12\x037\
    \x04\x0c\n\n\n\x03\x07\x12\x05\x12\x037\r\x11\n\n\n\x03\x07\x12\x01\x12\
    \x037\x12+\n\n\n\x03\x07\x12\x03\x12\x037.3\n\x93\x01\n\x02\x07\x13\x12\
    \x03:\x040\x1a\x87\x01\x20Use\x20`std::Option<T>`\x20to\x20store\x20sing\
    ular\x20messages\x20fields.\n\x20Note,\x20it's\x20not\x20possible\x20to\
    \x20have\x20recursive\x20messages\x20with\x20this\x20option\x20enabled.\
    \n\n\n\n\x03\x07\x13\x02\x12\x03'\x07%\n\n\n\x03\x07\x13\x04\x12\x03:\
    \x04\x0c\n\n\n\x03\x07\x13\x05\x12\x03:\r\x11\n\n\n\x03\x07\x13\x01\x12\
    \x03:\x12'\n\n\n\x03\x07\x13\x03\x12\x03:*/\nJ\n\x02\x07\x14\x12\x03<\
    \x04'\x1a?\x20Use\x20`serde_derive`\x20to\x20implement\x20`Serialize`\
    \x20and\x20`Deserialize`\n\n\n\n\x03\x07\x14\x02\x12\x03'\x07%\n\n\n\x03\
    \x07\x14\x04\x12\x03<\x04\x0c\n\n\n\x03\x07\x14\x05\x12\x03<\r\x11\n\n\n\
    \x03\x07\x14\x01\x12\x03<\x12\x1e\n\n\n\x03\x07\x14\x03\x12\x03<!&\n3\n\
    \x02\x07\x15\x12\x03>\x04-\x1a(\x20Guard\x20serde\x20annotations\x20with\
    \x20cfg\x20attr.\n\n\n\n\x03\x07\x15\x02\x12\x03'\x07%\n\n\n\x03\x07\x15\
    \x04\x12\x03>\x04\x0c\n\n\n\x03\x07\x15\x05\x12\x03>\r\x13\n\n\n\x03\x07\
    \x15\x01\x12\x03>\x14$\n\n\n\x03\x07\x15\x03\x12\x03>',\n\t\n\x01\x07\
    \x12\x04A\0S\x01\nI\n\x02\x07\x16\x12\x03C\x04.\x1a>\x20When\x20true\x20\
    all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\x20generated\
    \n\n\n\n\x03\x07\x16\x02\x12\x03A\x07#\n\n\n\x03\x07\x16\x04\x12\x03C\
    \x04\x0c\n\n\n\x03\x07\x16\x05\x12\x03C\r\x11\n\n\n\x03\x07\x16\x01\x12\
    \x03C\x12%\n\n\n\x03\x07\x16\x03\x12\x03C(-\nP\n\x02\x07\x17\x12\x03E\
    \x043\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20\
    accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x17\x02\x12\x03A\
    \x07#\n\n\n\x03\x07\x17\x04\x12\x03E\x04\x0c\n\n\n\x03\x07\x17\x05\x12\
    \x03E\r\x11\n\n\n\x03\x07\x17\x01\x12\x03E\x12*\n\n\n\x03\x07\x17\x03\
    \x12\x03E-2\nL\n\x02\x07\x18\x12\x03G\x040\x1aA\x20When\x20false,\x20`ge\
    t_`\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\
    \"`\n\n\n\n\x03\x07\x18\x02\x12\x03A\x07#\n\n\n\x03\x07\x18\x04\x12\x03G\
    \x04\x0c\n\n\n\x03\x07\x18\x05\x12\x03G\r\x11\n\n\n\x03\x07\x18\x01\x12\
    \x03G\x12'\n\n\n\x03\x07\x18\x03\x12\x03G*/\n2\n\x02\x07\x19\x12\x03I\
    \x04;\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\
    \x03\x07\x19\x02\x12\x03A\x07#\n\n\n\x03\x07\x19\x04\x12\x03I\x04\x0c\n\
    \n\n\x03\x07\x19\x05\x12\x03I\r\x11\n\n\n\x03\x07\x19\x01\x12\x03I\x122\
    \n\n\n\x03\x07\x19\x03\x12\x03I5:\n3\n\x02\x07\x1a\x12\x03K\x04<\x1a(\
    \x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\
    \x1a\x02\x12\x03A\x07#\n\n\n\x03\x07\x1a\x04\x12\x03K\x04\x0c\n\n\n\x03\
    \x07\x1a\x05\x12\x03K\r\x11\n\n\n\x03\x07\x1a\x01\x12\x03K\x123\n\n\n\
    \x03\x07\x1a\x03\x12\x03K6;\n<\n\x02\x07\x1b\x12\x03M\x043\x1a1\x20Use\
    \x20`std::Vec`\x20to\x20store\x20repeated\x20messages\x20field\n\n\n\n\
    \x03\x07\x1b\x02\x12\x03A\x07#\n\n\n\x03\x07\x1b\x04\x12\x03M\x04\x0c\n\
    \n\n\x03\x07\x1b\x05\x12\x03M\r\x11\n\n\n\x03\x07\x1b\x01\x12\x03M\x12*\
    \n\n\n\x03\x07\x1b\x03\x12\x03M-2\nM\n\x02\x07\x1c\x12\x03O\x04:\x1aB\
    \x20Use\x20`std::Option<std::Box<T>>`\x20to\x20store\x20singular\x20mess\
    ages\x20fields\n\n\n\n\x03\x07\x1c\x02\x12\x03A\x07#\n\n\n\x03\x07\x1c\
    \x04\x12\x03O\x04\x0c\n\n\n\x03\x07\x1c\x05\x12\x03O\r\x11\n\n\n\x03\x07\
    \x1c\x01\x12\x03O\x121\n\n\n\x03\x07\x1c\x03\x12\x03O49\n\x93\x01\n\x02\
    \x07\x1d\x12\x03R\x046\x1a\x87\x01\x20Use\x20`std::Option<T>`\x20to\x20s\
    tore\x20singular\x20messages\x20fields.\n\x20Note,\x20it's\x20not\x20pos\
    sible\x20to\x20have\x20recursive\x20messages\x20with\x20this\x20option\
    \x20enabled.\n\n\n\n\x03\x07\x1d\x02\x12\x03A\x07#\n\n\n\x03\x07\x1d\x04\
    \x12\x03R\x04\x0c\n\n\n\x03\x07\x1d\x05\x12\x03R\r\x11\n\n\n\x03\x07\x1d\
    \x01\x12\x03R\x12-\n\n\n\x03\x07\x1d\x03\x12\x03R05\
";

static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
