// This file is generated. Do not edit
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
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Account {
    // message fields
    id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Account {}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Account {
        static mut instance: ::protobuf::lazy::Lazy<Account> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Account,
        };
        unsafe {
            instance.get(Account::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for Account {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.email.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Account {
    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static(_: ::std::option::Option<Account>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Account::get_id_for_reflect,
                    Account::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    Account::get_email_for_reflect,
                    Account::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Account::get_name_for_reflect,
                    Account::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Account>(
                    "Account",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Account {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_email();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Account {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountFindOrCreate {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    email: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountFindOrCreate {}

impl AccountFindOrCreate {
    pub fn new() -> AccountFindOrCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountFindOrCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountFindOrCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountFindOrCreate,
        };
        unsafe {
            instance.get(AccountFindOrCreate::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }
}

impl ::protobuf::Message for AccountFindOrCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.email.as_ref() {
            os.write_string(2, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountFindOrCreate {
    fn new() -> AccountFindOrCreate {
        AccountFindOrCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountFindOrCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AccountFindOrCreate::get_name_for_reflect,
                    AccountFindOrCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    AccountFindOrCreate::get_email_for_reflect,
                    AccountFindOrCreate::mut_email_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountFindOrCreate>(
                    "AccountFindOrCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountFindOrCreate {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_email();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountFindOrCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountFindOrCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountGet {}

impl AccountGet {
    pub fn new() -> AccountGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountGet {
        static mut instance: ::protobuf::lazy::Lazy<AccountGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountGet,
        };
        unsafe {
            instance.get(AccountGet::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for AccountGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountGet {
    fn new() -> AccountGet {
        AccountGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AccountGet::get_name_for_reflect,
                    AccountGet::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountGet>(
                    "AccountGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountGet {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountGetId {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountGetId {}

impl AccountGetId {
    pub fn new() -> AccountGetId {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountGetId {
        static mut instance: ::protobuf::lazy::Lazy<AccountGetId> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountGetId,
        };
        unsafe {
            instance.get(AccountGetId::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }
}

impl ::protobuf::Message for AccountGetId {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountGetId {
    fn new() -> AccountGetId {
        AccountGetId::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountGetId>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    AccountGetId::get_id_for_reflect,
                    AccountGetId::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountGetId>(
                    "AccountGetId",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountGetId {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountGetId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountGetId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountCreate {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    email: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountCreate {}

impl AccountCreate {
    pub fn new() -> AccountCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountCreate,
        };
        unsafe {
            instance.get(AccountCreate::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }
}

impl ::protobuf::Message for AccountCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.email.as_ref() {
            os.write_string(2, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountCreate {
    fn new() -> AccountCreate {
        AccountCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AccountCreate::get_name_for_reflect,
                    AccountCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    AccountCreate::get_email_for_reflect,
                    AccountCreate::mut_email_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountCreate>(
                    "AccountCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountCreate {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_email();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitation {
    // message fields
    id: ::std::option::Option<u64>,
    origin_invitation_id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitation {}

impl AccountOriginInvitation {
    pub fn new() -> AccountOriginInvitation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitation {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitation,
        };
        unsafe {
            instance.get(AccountOriginInvitation::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional uint64 origin_invitation_id = 2;

    pub fn clear_origin_invitation_id(&mut self) {
        self.origin_invitation_id = ::std::option::Option::None;
    }

    pub fn has_origin_invitation_id(&self) -> bool {
        self.origin_invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_invitation_id(&mut self, v: u64) {
        self.origin_invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_invitation_id(&self) -> u64 {
        self.origin_invitation_id.unwrap_or(0)
    }

    fn get_origin_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_invitation_id
    }

    fn mut_origin_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_invitation_id
    }

    // optional uint64 account_id = 3;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 4;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        };
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 5;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 6;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        };
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional uint64 owner_id = 7;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for AccountOriginInvitation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_invitation_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_invitation_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.origin_invitation_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.account_id {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.account_name.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.origin_id {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.origin_name.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(7, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitation {
    fn new() -> AccountOriginInvitation {
        AccountOriginInvitation::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    AccountOriginInvitation::get_id_for_reflect,
                    AccountOriginInvitation::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_invitation_id",
                    AccountOriginInvitation::get_origin_invitation_id_for_reflect,
                    AccountOriginInvitation::mut_origin_invitation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitation::get_account_id_for_reflect,
                    AccountOriginInvitation::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginInvitation::get_account_name_for_reflect,
                    AccountOriginInvitation::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginInvitation::get_origin_id_for_reflect,
                    AccountOriginInvitation::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginInvitation::get_origin_name_for_reflect,
                    AccountOriginInvitation::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    AccountOriginInvitation::get_owner_id_for_reflect,
                    AccountOriginInvitation::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitation>(
                    "AccountOriginInvitation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_invitation_id();
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationCreate {
    // message fields
    origin_invitation_id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationCreate {}

impl AccountOriginInvitationCreate {
    pub fn new() -> AccountOriginInvitationCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationCreate,
        };
        unsafe {
            instance.get(AccountOriginInvitationCreate::new)
        }
    }

    // optional uint64 origin_invitation_id = 1;

    pub fn clear_origin_invitation_id(&mut self) {
        self.origin_invitation_id = ::std::option::Option::None;
    }

    pub fn has_origin_invitation_id(&self) -> bool {
        self.origin_invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_invitation_id(&mut self, v: u64) {
        self.origin_invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_invitation_id(&self) -> u64 {
        self.origin_invitation_id.unwrap_or(0)
    }

    fn get_origin_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_invitation_id
    }

    fn mut_origin_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_invitation_id
    }

    // optional uint64 account_id = 2;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 3;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        };
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 4;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 5;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        };
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional uint64 owner_id = 6;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for AccountOriginInvitationCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_invitation_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.origin_invitation_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_invitation_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.account_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.account_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.origin_id {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.origin_name.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(6, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationCreate {
    fn new() -> AccountOriginInvitationCreate {
        AccountOriginInvitationCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_invitation_id",
                    AccountOriginInvitationCreate::get_origin_invitation_id_for_reflect,
                    AccountOriginInvitationCreate::mut_origin_invitation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationCreate::get_account_id_for_reflect,
                    AccountOriginInvitationCreate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginInvitationCreate::get_account_name_for_reflect,
                    AccountOriginInvitationCreate::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginInvitationCreate::get_origin_id_for_reflect,
                    AccountOriginInvitationCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginInvitationCreate::get_origin_name_for_reflect,
                    AccountOriginInvitationCreate::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    AccountOriginInvitationCreate::get_owner_id_for_reflect,
                    AccountOriginInvitationCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationCreate>(
                    "AccountOriginInvitationCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationCreate {
    fn clear(&mut self) {
        self.clear_origin_invitation_id();
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationAcceptRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    invite_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    ignore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationAcceptRequest {}

impl AccountOriginInvitationAcceptRequest {
    pub fn new() -> AccountOriginInvitationAcceptRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationAcceptRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationAcceptRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationAcceptRequest,
        };
        unsafe {
            instance.get(AccountOriginInvitationAcceptRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint64 invite_id = 2;

    pub fn clear_invite_id(&mut self) {
        self.invite_id = ::std::option::Option::None;
    }

    pub fn has_invite_id(&self) -> bool {
        self.invite_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invite_id(&mut self, v: u64) {
        self.invite_id = ::std::option::Option::Some(v);
    }

    pub fn get_invite_id(&self) -> u64 {
        self.invite_id.unwrap_or(0)
    }

    fn get_invite_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invite_id
    }

    fn mut_invite_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invite_id
    }

    // optional string origin_name = 3;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        };
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional bool ignore = 4;

    pub fn clear_ignore(&mut self) {
        self.ignore = ::std::option::Option::None;
    }

    pub fn has_ignore(&self) -> bool {
        self.ignore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignore(&mut self, v: bool) {
        self.ignore = ::std::option::Option::Some(v);
    }

    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap_or(false)
    }

    fn get_ignore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ignore
    }

    fn mut_ignore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ignore
    }
}

impl ::protobuf::Message for AccountOriginInvitationAcceptRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.invite_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.ignore = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.invite_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.ignore {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.invite_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.origin_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.ignore {
            os.write_bool(4, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationAcceptRequest {
    fn new() -> AccountOriginInvitationAcceptRequest {
        AccountOriginInvitationAcceptRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationAcceptRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationAcceptRequest::get_account_id_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invite_id",
                    AccountOriginInvitationAcceptRequest::get_invite_id_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_invite_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginInvitationAcceptRequest::get_origin_name_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore",
                    AccountOriginInvitationAcceptRequest::get_ignore_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_ignore_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationAcceptRequest>(
                    "AccountOriginInvitationAcceptRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationAcceptRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invite_id();
        self.clear_origin_name();
        self.clear_ignore();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationAcceptRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationAcceptRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationIgnoreRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitation_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationIgnoreRequest {}

impl AccountOriginInvitationIgnoreRequest {
    pub fn new() -> AccountOriginInvitationIgnoreRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationIgnoreRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationIgnoreRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationIgnoreRequest,
        };
        unsafe {
            instance.get(AccountOriginInvitationIgnoreRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint64 invitation_id = 2;

    pub fn clear_invitation_id(&mut self) {
        self.invitation_id = ::std::option::Option::None;
    }

    pub fn has_invitation_id(&self) -> bool {
        self.invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invitation_id(&mut self, v: u64) {
        self.invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_invitation_id(&self) -> u64 {
        self.invitation_id.unwrap_or(0)
    }

    fn get_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invitation_id
    }

    fn mut_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invitation_id
    }
}

impl ::protobuf::Message for AccountOriginInvitationIgnoreRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.invitation_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.invitation_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.invitation_id {
            os.write_uint64(2, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationIgnoreRequest {
    fn new() -> AccountOriginInvitationIgnoreRequest {
        AccountOriginInvitationIgnoreRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationIgnoreRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationIgnoreRequest::get_account_id_for_reflect,
                    AccountOriginInvitationIgnoreRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invitation_id",
                    AccountOriginInvitationIgnoreRequest::get_invitation_id_for_reflect,
                    AccountOriginInvitationIgnoreRequest::mut_invitation_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationIgnoreRequest>(
                    "AccountOriginInvitationIgnoreRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationIgnoreRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitation_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationIgnoreRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationIgnoreRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationRescindRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitation_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationRescindRequest {}

impl AccountOriginInvitationRescindRequest {
    pub fn new() -> AccountOriginInvitationRescindRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationRescindRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationRescindRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationRescindRequest,
        };
        unsafe {
            instance.get(AccountOriginInvitationRescindRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint64 invitation_id = 2;

    pub fn clear_invitation_id(&mut self) {
        self.invitation_id = ::std::option::Option::None;
    }

    pub fn has_invitation_id(&self) -> bool {
        self.invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invitation_id(&mut self, v: u64) {
        self.invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_invitation_id(&self) -> u64 {
        self.invitation_id.unwrap_or(0)
    }

    fn get_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invitation_id
    }

    fn mut_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invitation_id
    }
}

impl ::protobuf::Message for AccountOriginInvitationRescindRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.invitation_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.invitation_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.invitation_id {
            os.write_uint64(2, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationRescindRequest {
    fn new() -> AccountOriginInvitationRescindRequest {
        AccountOriginInvitationRescindRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationRescindRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationRescindRequest::get_account_id_for_reflect,
                    AccountOriginInvitationRescindRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invitation_id",
                    AccountOriginInvitationRescindRequest::get_invitation_id_for_reflect,
                    AccountOriginInvitationRescindRequest::mut_invitation_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationRescindRequest>(
                    "AccountOriginInvitationRescindRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationRescindRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitation_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationRescindRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationRescindRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInvitationListRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListRequest {}

impl AccountInvitationListRequest {
    pub fn new() -> AccountInvitationListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListRequest,
        };
        unsafe {
            instance.get(AccountInvitationListRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountInvitationListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountInvitationListRequest {
    fn new() -> AccountInvitationListRequest {
        AccountInvitationListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListRequest::get_account_id_for_reflect,
                    AccountInvitationListRequest::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListRequest>(
                    "AccountInvitationListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInvitationListResponse {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitations: ::protobuf::RepeatedField<AccountOriginInvitation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListResponse {}

impl AccountInvitationListResponse {
    pub fn new() -> AccountInvitationListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListResponse,
        };
        unsafe {
            instance.get(AccountInvitationListResponse::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // repeated .sessionsrv.AccountOriginInvitation invitations = 2;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<AccountOriginInvitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<AccountOriginInvitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<AccountOriginInvitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[AccountOriginInvitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<AccountOriginInvitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AccountOriginInvitation> {
        &mut self.invitations
    }
}

impl ::protobuf::Message for AccountInvitationListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.invitations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountInvitationListResponse {
    fn new() -> AccountInvitationListResponse {
        AccountInvitationListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListResponse::get_account_id_for_reflect,
                    AccountInvitationListResponse::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AccountOriginInvitation>>(
                    "invitations",
                    AccountInvitationListResponse::get_invitations_for_reflect,
                    AccountInvitationListResponse::mut_invitations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListResponse>(
                    "AccountInvitationListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListResponse {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginCreate {
    // message fields
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginCreate {}

impl AccountOriginCreate {
    pub fn new() -> AccountOriginCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginCreate,
        };
        unsafe {
            instance.get(AccountOriginCreate::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 2;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        };
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 3;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 4;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        };
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }
}

impl ::protobuf::Message for AccountOriginCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.account_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.origin_id {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.origin_name.as_ref() {
            os.write_string(4, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginCreate {
    fn new() -> AccountOriginCreate {
        AccountOriginCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginCreate::get_account_id_for_reflect,
                    AccountOriginCreate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginCreate::get_account_name_for_reflect,
                    AccountOriginCreate::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginCreate::get_origin_id_for_reflect,
                    AccountOriginCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginCreate::get_origin_name_for_reflect,
                    AccountOriginCreate::mut_origin_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginCreate>(
                    "AccountOriginCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginCreate {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginListRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginListRequest {}

impl AccountOriginListRequest {
    pub fn new() -> AccountOriginListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginListRequest,
        };
        unsafe {
            instance.get(AccountOriginListRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountOriginListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginListRequest {
    fn new() -> AccountOriginListRequest {
        AccountOriginListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginListRequest::get_account_id_for_reflect,
                    AccountOriginListRequest::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginListRequest>(
                    "AccountOriginListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginListRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginListResponse {
    // message fields
    account_id: ::std::option::Option<u64>,
    origins: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginListResponse {}

impl AccountOriginListResponse {
    pub fn new() -> AccountOriginListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginListResponse,
        };
        unsafe {
            instance.get(AccountOriginListResponse::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // repeated string origins = 2;

    pub fn clear_origins(&mut self) {
        self.origins.clear();
    }

    // Param is passed by value, moved
    pub fn set_origins(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.origins = v;
    }

    // Mutable pointer to the field.
    pub fn mut_origins(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.origins
    }

    // Take field
    pub fn take_origins(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.origins, ::protobuf::RepeatedField::new())
    }

    pub fn get_origins(&self) -> &[::std::string::String] {
        &self.origins
    }

    fn get_origins_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.origins
    }

    fn mut_origins_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.origins
    }
}

impl ::protobuf::Message for AccountOriginListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.origins)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.origins {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.origins {
            os.write_string(2, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginListResponse {
    fn new() -> AccountOriginListResponse {
        AccountOriginListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginListResponse::get_account_id_for_reflect,
                    AccountOriginListResponse::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origins",
                    AccountOriginListResponse::get_origins_for_reflect,
                    AccountOriginListResponse::mut_origins_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginListResponse>(
                    "AccountOriginListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginListResponse {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_origins();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginRemove {
    // message fields
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginRemove {}

impl AccountOriginRemove {
    pub fn new() -> AccountOriginRemove {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginRemove {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginRemove> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginRemove,
        };
        unsafe {
            instance.get(AccountOriginRemove::new)
        }
    }

    // optional string account_name = 1;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        };
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for AccountOriginRemove {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginRemove {
    fn new() -> AccountOriginRemove {
        AccountOriginRemove::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginRemove>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginRemove::get_account_name_for_reflect,
                    AccountOriginRemove::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginRemove::get_origin_id_for_reflect,
                    AccountOriginRemove::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginRemove>(
                    "AccountOriginRemove",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginRemove {
    fn clear(&mut self) {
        self.clear_account_name();
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginRemove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginRemove {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Session {
    // message fields
    id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    token: ::protobuf::SingularField<::std::string::String>,
    flags: ::std::option::Option<u32>,
    oauth_token: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Session {}

impl Session {
    pub fn new() -> Session {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Session {
        static mut instance: ::protobuf::lazy::Lazy<Session> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Session,
        };
        unsafe {
            instance.get(Session::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string token = 4;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token
    }

    // optional uint32 flags = 5;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }

    // optional string oauth_token = 6;

    pub fn clear_oauth_token(&mut self) {
        self.oauth_token.clear();
    }

    pub fn has_oauth_token(&self) -> bool {
        self.oauth_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oauth_token(&mut self, v: ::std::string::String) {
        self.oauth_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oauth_token(&mut self) -> &mut ::std::string::String {
        if self.oauth_token.is_none() {
            self.oauth_token.set_default();
        };
        self.oauth_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_oauth_token(&mut self) -> ::std::string::String {
        self.oauth_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_oauth_token(&self) -> &str {
        match self.oauth_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_oauth_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.oauth_token
    }

    fn mut_oauth_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.oauth_token
    }
}

impl ::protobuf::Message for Session {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.oauth_token)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.oauth_token.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.email.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.token.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.oauth_token.as_ref() {
            os.write_string(6, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Session {
    fn new() -> Session {
        Session::new()
    }

    fn descriptor_static(_: ::std::option::Option<Session>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Session::get_id_for_reflect,
                    Session::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    Session::get_email_for_reflect,
                    Session::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Session::get_name_for_reflect,
                    Session::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    Session::get_token_for_reflect,
                    Session::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    Session::get_flags_for_reflect,
                    Session::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "oauth_token",
                    Session::get_oauth_token_for_reflect,
                    Session::mut_oauth_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Session>(
                    "Session",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Session {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_email();
        self.clear_name();
        self.clear_token();
        self.clear_flags();
        self.clear_oauth_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Session {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Session {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionCreate {
    // message fields
    token: ::protobuf::SingularField<::std::string::String>,
    extern_id: ::std::option::Option<u32>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    provider: ::std::option::Option<OAuthProvider>,
    app_token: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionCreate {}

impl SessionCreate {
    pub fn new() -> SessionCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionCreate {
        static mut instance: ::protobuf::lazy::Lazy<SessionCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionCreate,
        };
        unsafe {
            instance.get(SessionCreate::new)
        }
    }

    // optional string token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token
    }

    // optional uint32 extern_id = 2;

    pub fn clear_extern_id(&mut self) {
        self.extern_id = ::std::option::Option::None;
    }

    pub fn has_extern_id(&self) -> bool {
        self.extern_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extern_id(&mut self, v: u32) {
        self.extern_id = ::std::option::Option::Some(v);
    }

    pub fn get_extern_id(&self) -> u32 {
        self.extern_id.unwrap_or(0)
    }

    fn get_extern_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.extern_id
    }

    fn mut_extern_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.extern_id
    }

    // optional string email = 3;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }

    // optional string name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .sessionsrv.OAuthProvider provider = 5;

    pub fn clear_provider(&mut self) {
        self.provider = ::std::option::Option::None;
    }

    pub fn has_provider(&self) -> bool {
        self.provider.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.unwrap_or(OAuthProvider::GitHub)
    }

    fn get_provider_for_reflect(&self) -> &::std::option::Option<OAuthProvider> {
        &self.provider
    }

    fn mut_provider_for_reflect(&mut self) -> &mut ::std::option::Option<OAuthProvider> {
        &mut self.provider
    }

    // optional string app_token = 6;

    pub fn clear_app_token(&mut self) {
        self.app_token.clear();
    }

    pub fn has_app_token(&self) -> bool {
        self.app_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_token(&mut self, v: ::std::string::String) {
        self.app_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_token(&mut self) -> &mut ::std::string::String {
        if self.app_token.is_none() {
            self.app_token.set_default();
        };
        self.app_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_app_token(&mut self) -> ::std::string::String {
        self.app_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_app_token(&self) -> &str {
        match self.app_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_app_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.app_token
    }

    fn mut_app_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.app_token
    }
}

impl ::protobuf::Message for SessionCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.extern_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.provider = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.app_token)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.extern_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.provider {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        if let Some(v) = self.app_token.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.token.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.extern_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.email.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.provider {
            os.write_enum(5, v.value())?;
        };
        if let Some(v) = self.app_token.as_ref() {
            os.write_string(6, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionCreate {
    fn new() -> SessionCreate {
        SessionCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    SessionCreate::get_token_for_reflect,
                    SessionCreate::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "extern_id",
                    SessionCreate::get_extern_id_for_reflect,
                    SessionCreate::mut_extern_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    SessionCreate::get_email_for_reflect,
                    SessionCreate::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    SessionCreate::get_name_for_reflect,
                    SessionCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OAuthProvider>>(
                    "provider",
                    SessionCreate::get_provider_for_reflect,
                    SessionCreate::mut_provider_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "app_token",
                    SessionCreate::get_app_token_for_reflect,
                    SessionCreate::mut_app_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionCreate>(
                    "SessionCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionCreate {
    fn clear(&mut self) {
        self.clear_token();
        self.clear_extern_id();
        self.clear_email();
        self.clear_name();
        self.clear_provider();
        self.clear_app_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionGet {
    // message fields
    token: ::protobuf::SingularPtrField<SessionToken>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionGet {}

impl SessionGet {
    pub fn new() -> SessionGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionGet {
        static mut instance: ::protobuf::lazy::Lazy<SessionGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionGet,
        };
        unsafe {
            instance.get(SessionGet::new)
        }
    }

    // optional .sessionsrv.SessionToken token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: SessionToken) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut SessionToken {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> SessionToken {
        self.token.take().unwrap_or_else(|| SessionToken::new())
    }

    pub fn get_token(&self) -> &SessionToken {
        self.token.as_ref().unwrap_or_else(|| SessionToken::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<SessionToken> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SessionToken> {
        &mut self.token
    }
}

impl ::protobuf::Message for SessionGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.token)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.token.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionGet {
    fn new() -> SessionGet {
        SessionGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionToken>>(
                    "token",
                    SessionGet::get_token_for_reflect,
                    SessionGet::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionGet>(
                    "SessionGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionGet {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionToken {
    // message fields
    account_id: ::std::option::Option<u64>,
    extern_id: ::std::option::Option<u32>,
    provider: ::std::option::Option<OAuthProvider>,
    token: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionToken {}

impl SessionToken {
    pub fn new() -> SessionToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionToken {
        static mut instance: ::protobuf::lazy::Lazy<SessionToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionToken,
        };
        unsafe {
            instance.get(SessionToken::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint32 extern_id = 2;

    pub fn clear_extern_id(&mut self) {
        self.extern_id = ::std::option::Option::None;
    }

    pub fn has_extern_id(&self) -> bool {
        self.extern_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extern_id(&mut self, v: u32) {
        self.extern_id = ::std::option::Option::Some(v);
    }

    pub fn get_extern_id(&self) -> u32 {
        self.extern_id.unwrap_or(0)
    }

    fn get_extern_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.extern_id
    }

    fn mut_extern_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.extern_id
    }

    // optional .sessionsrv.OAuthProvider provider = 3;

    pub fn clear_provider(&mut self) {
        self.provider = ::std::option::Option::None;
    }

    pub fn has_provider(&self) -> bool {
        self.provider.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.unwrap_or(OAuthProvider::GitHub)
    }

    fn get_provider_for_reflect(&self) -> &::std::option::Option<OAuthProvider> {
        &self.provider
    }

    fn mut_provider_for_reflect(&mut self) -> &mut ::std::option::Option<OAuthProvider> {
        &mut self.provider
    }

    // optional bytes token = 4;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::vec::Vec<u8>) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::vec::Vec<u8> {
        self.token.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_token(&self) -> &[u8] {
        match self.token.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.token
    }
}

impl ::protobuf::Message for SessionToken {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.extern_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.provider = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.token)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.extern_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.provider {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if let Some(v) = self.token.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.extern_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.provider {
            os.write_enum(3, v.value())?;
        };
        if let Some(v) = self.token.as_ref() {
            os.write_bytes(4, &v)?;
        };
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionToken {
    fn new() -> SessionToken {
        SessionToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    SessionToken::get_account_id_for_reflect,
                    SessionToken::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "extern_id",
                    SessionToken::get_extern_id_for_reflect,
                    SessionToken::mut_extern_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OAuthProvider>>(
                    "provider",
                    SessionToken::get_provider_for_reflect,
                    SessionToken::mut_provider_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "token",
                    SessionToken::get_token_for_reflect,
                    SessionToken::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionToken>(
                    "SessionToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionToken {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_extern_id();
        self.clear_provider();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionToken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OAuthProvider {
    GitHub = 0,
}

impl ::protobuf::ProtobufEnum for OAuthProvider {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OAuthProvider> {
        match value {
            0 => ::std::option::Option::Some(OAuthProvider::GitHub),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OAuthProvider] = &[
            OAuthProvider::GitHub,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<OAuthProvider>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OAuthProvider", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OAuthProvider {
}

impl ::protobuf::reflect::ProtobufValue for OAuthProvider {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x73, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x73, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x72, 0x76, 0x22, 0x43, 0x0a, 0x07, 0x41, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x02, 0x69, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x3f, 0x0a,
    0x13, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x4f, 0x72, 0x43, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69,
    0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x22, 0x20,
    0x0a, 0x0a, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x47, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x22, 0x1e, 0x0a, 0x0c, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x47, 0x65, 0x74, 0x49, 0x64,
    0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64,
    0x22, 0x39, 0x0a, 0x0d, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x22, 0xf6, 0x01, 0x0a, 0x17,
    0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76,
    0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x5f, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x12, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76,
    0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x49, 0x64, 0x22, 0xec, 0x01, 0x0a, 0x1d, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x30, 0x0a, 0x14, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x5f, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x12, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x49, 0x64, 0x22, 0x9b, 0x01, 0x0a, 0x24, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x41,
    0x63, 0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x69,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08,
    0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x69, 0x67, 0x6e,
    0x6f, 0x72, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x69, 0x67, 0x6e, 0x6f, 0x72,
    0x65, 0x22, 0x6a, 0x0a, 0x24, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x67, 0x6e, 0x6f,
    0x72, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x69, 0x6e, 0x76, 0x69,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0c, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x6b, 0x0a,
    0x25, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e,
    0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x63, 0x69, 0x6e, 0x64, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x69, 0x6e,
    0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x3d, 0x0a, 0x1c, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c,
    0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0x85, 0x01, 0x0a, 0x1d, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c,
    0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x45, 0x0a, 0x0b, 0x69, 0x6e,
    0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x23, 0x2e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x41, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x22, 0x95, 0x01, 0x0a, 0x13, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x39, 0x0a, 0x18, 0x41, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x22, 0x54, 0x0a, 0x19, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64,
    0x12, 0x18, 0x0a, 0x07, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x07, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x73, 0x22, 0x55, 0x0a, 0x13, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x52, 0x65, 0x6d, 0x6f, 0x76,
    0x65, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49,
    0x64, 0x22, 0x90, 0x01, 0x0a, 0x07, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x0e, 0x0a,
    0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x14, 0x0a,
    0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6d,
    0x61, 0x69, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x14, 0x0a,
    0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c,
    0x61, 0x67, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x6f, 0x61, 0x75, 0x74, 0x68, 0x54,
    0x6f, 0x6b, 0x65, 0x6e, 0x22, 0xc0, 0x01, 0x0a, 0x0d, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x1b, 0x0a, 0x09,
    0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x08, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61,
    0x69, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x35, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x72, 0x76, 0x2e, 0x4f, 0x41, 0x75, 0x74, 0x68, 0x50, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72,
    0x52, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x12, 0x1b, 0x0a, 0x09, 0x61, 0x70,
    0x70, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x61,
    0x70, 0x70, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x3c, 0x0a, 0x0a, 0x53, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x47, 0x65, 0x74, 0x12, 0x2e, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x72,
    0x76, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x52, 0x05,
    0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x97, 0x01, 0x0a, 0x0c, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e,
    0x49, 0x64, 0x12, 0x35, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x72,
    0x76, 0x2e, 0x4f, 0x41, 0x75, 0x74, 0x68, 0x50, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x52,
    0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2a,
    0x1b, 0x0a, 0x0d, 0x4f, 0x41, 0x75, 0x74, 0x68, 0x50, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72,
    0x12, 0x0a, 0x0a, 0x06, 0x47, 0x69, 0x74, 0x48, 0x75, 0x62, 0x10, 0x00, 0x4a, 0xfd, 0x24, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x7e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05,
    0x00, 0x12, 0x04, 0x03, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03,
    0x03, 0x05, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x04, 0x0b, 0x0c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x07, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x09, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0a, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x10, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0f, 0x1a, 0x1b, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x13, 0x00, 0x15,
    0x01, 0x1a, 0x1f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x47, 0x48, 0x20, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d,
    0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x12, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x14, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x14, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x17, 0x00, 0x19, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x17, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18,
    0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x17, 0x18,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1b, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x1c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x1d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1d, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d,
    0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x20, 0x00, 0x28, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x20, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x12,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x22, 0x12, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x22, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x23, 0x02,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x03, 0x12, 0x03, 0x24, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x24,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x12, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x21, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x25, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x04, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x25, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x25, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x26, 0x02, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x26, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x26, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x06,
    0x12, 0x03, 0x27, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x05, 0x12, 0x03, 0x27, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x01, 0x12, 0x03, 0x27, 0x12, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03, 0x12, 0x03, 0x27, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x2a, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x2a, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2b,
    0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x12, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x12,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x2d, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x2d, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x2d, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2e, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x04, 0x12, 0x03, 0x2f, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2f,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2f, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x30, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x30, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x30, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x33, 0x00, 0x38, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x33, 0x08, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x34, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x34, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x35, 0x02, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x35, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x35, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x35, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x36,
    0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x36, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x36, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x36, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x03, 0x12, 0x03, 0x37, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x37, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x37, 0x10,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x37, 0x19, 0x1a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x3a, 0x00, 0x3d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x08, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12,
    0x03, 0x3b, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x3c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x3c, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x22,
    0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x3f, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x03, 0x40, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x40,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x41, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x41, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x41, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x41, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x44, 0x00, 0x46, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x44, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x00, 0x12, 0x03, 0x45, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x45, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x45, 0x1f, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x48, 0x00, 0x4b, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0b, 0x01, 0x12, 0x03, 0x48, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00,
    0x12, 0x03, 0x49, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x49, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49, 0x12, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x4a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x4a, 0x0b, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x4a, 0x23, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4a,
    0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x4d, 0x00, 0x52, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x00, 0x12, 0x03, 0x4e, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x4e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x12,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x4f, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x4f, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x50, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x50, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x50, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x50, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x03, 0x12, 0x03, 0x51, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x51, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x03, 0x51,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x03, 0x51, 0x12, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x03, 0x51, 0x20, 0x21, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x54, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x03, 0x54, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03,
    0x55, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x55, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x12, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0e, 0x12, 0x04, 0x58, 0x00, 0x5b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03,
    0x58, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x59, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x59, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x59, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01,
    0x12, 0x03, 0x5a, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x5a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5a, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5a, 0x12, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5a, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0f, 0x12, 0x04, 0x5d, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01,
    0x12, 0x03, 0x5d, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x5e,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5e, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5e, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x01, 0x12, 0x03, 0x5f, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x5f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5f, 0x12,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5f, 0x1e, 0x1f, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x62, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x10, 0x01, 0x12, 0x03, 0x62, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12,
    0x03, 0x63, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x12, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x64, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x64, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x64, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x64, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x03, 0x65, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x03, 0x65, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x03, 0x65, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x65, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x65, 0x19, 0x1a, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x03,
    0x67, 0x02, 0x1c, 0x1a, 0x36, 0x20, 0x62, 0x61, 0x73, 0x65, 0x36, 0x34, 0x20, 0x65, 0x6e, 0x63,
    0x6f, 0x64, 0x65, 0x64, 0x20, 0x62, 0x69, 0x6e, 0x61, 0x72, 0x79, 0x20, 0x72, 0x65, 0x70, 0x72,
    0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x03, 0x04, 0x12, 0x03, 0x67, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x67, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x67, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x67, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x03, 0x68, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12, 0x03, 0x68, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x05, 0x12, 0x03, 0x68, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x03, 0x68, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x68, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x05,
    0x12, 0x03, 0x69, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x69, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x05, 0x12, 0x03, 0x69, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12, 0x03, 0x69, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x03, 0x69, 0x20, 0x21, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x11, 0x12, 0x04, 0x6c, 0x00, 0x73, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01,
    0x12, 0x03, 0x6c, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x03, 0x6d,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x01, 0x12, 0x03, 0x6e, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x6e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x6e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6e, 0x12,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6e, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x03, 0x6f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x6f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x6f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x6f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x03, 0x70, 0x02,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x05, 0x12, 0x03, 0x70, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x03, 0x01, 0x12, 0x03, 0x70, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x03, 0x03, 0x12, 0x03, 0x70, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x04, 0x12, 0x03, 0x71, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x71, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x06, 0x12, 0x03, 0x71,
    0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x01, 0x12, 0x03, 0x71, 0x19, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x03, 0x12, 0x03, 0x71, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x05, 0x12, 0x03, 0x72, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x05, 0x04, 0x12, 0x03, 0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x72, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x72, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x72, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x04, 0x75, 0x00, 0x77, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x03, 0x75, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x12, 0x02, 0x00, 0x12, 0x03, 0x76, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x76, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x76, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76,
    0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76, 0x20, 0x21,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x04, 0x79, 0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x13, 0x01, 0x12, 0x03, 0x79, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00,
    0x12, 0x03, 0x7a, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7a, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a, 0x12, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x03, 0x7b, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x7b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x7b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x7b, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7b,
    0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x03, 0x7c, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7c, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x7c, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03, 0x12,
    0x03, 0x7d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7d, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7d, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7d, 0x19, 0x1a,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
