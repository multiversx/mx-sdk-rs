#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
use core::num;
use multiversx_sc::imports::*;
use serde::{Deserialize, Serialize};
const BUFFER_SIZE: usize = 200;
pub struct SerdeStruct1 {
    pub value1: u32,
    pub value2: u32,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SerdeStruct1 {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SerdeStruct1",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "value1",
                &self.value1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "value2",
                &self.value2,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SerdeStruct1 {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "value1" => _serde::__private::Ok(__Field::__field0),
                        "value2" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"value1" => _serde::__private::Ok(__Field::__field0),
                        b"value2" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SerdeStruct1>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SerdeStruct1;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SerdeStruct1",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        u32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SerdeStruct1 with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        u32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SerdeStruct1 with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SerdeStruct1 {
                        value1: __field0,
                        value2: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value1"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value2"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("value1")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("value2")?
                        }
                    };
                    _serde::__private::Ok(SerdeStruct1 {
                        value1: __field0,
                        value2: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["value1", "value2"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SerdeStruct1",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SerdeStruct1>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
pub struct SerdeStruct2 {
    pub big_int: num_bigint::BigInt,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SerdeStruct2 {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SerdeStruct2",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "big_int",
                &self.big_int,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SerdeStruct2 {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "big_int" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"big_int" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SerdeStruct2>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SerdeStruct2;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SerdeStruct2",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        num_bigint::BigInt,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SerdeStruct2 with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SerdeStruct2 { big_int: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<num_bigint::BigInt> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "big_int",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        num_bigint::BigInt,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("big_int")?
                        }
                    };
                    _serde::__private::Ok(SerdeStruct2 { big_int: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["big_int"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SerdeStruct2",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SerdeStruct2>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
pub struct ManagedSerdeStruct<M: ManagedTypeApi> {
    mb: ManagedBuffer<M>,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<M: ManagedTypeApi> _serde::Serialize for ManagedSerdeStruct<M>
    where
        M: _serde::Serialize,
    {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "ManagedSerdeStruct",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "mb",
                &self.mb,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, M: ManagedTypeApi> _serde::Deserialize<'de> for ManagedSerdeStruct<M>
    where
        M: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "mb" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"mb" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, M: ManagedTypeApi>
            where
                M: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<ManagedSerdeStruct<M>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, M: ManagedTypeApi> _serde::de::Visitor<'de> for __Visitor<'de, M>
            where
                M: _serde::Deserialize<'de>,
            {
                type Value = ManagedSerdeStruct<M>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct ManagedSerdeStruct",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        ManagedBuffer<M>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ManagedSerdeStruct with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(ManagedSerdeStruct { mb: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<ManagedBuffer<M>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mb"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ManagedBuffer<M>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("mb")?
                        }
                    };
                    _serde::__private::Ok(ManagedSerdeStruct { mb: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["mb"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ManagedSerdeStruct",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ManagedSerdeStruct<M>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
fn some_bigint() -> num_bigint::BigInt {
    num_bigint::BigInt::from(12345678901234567890u128)
}
pub trait SerdeFeatures: multiversx_sc::contract_base::ContractBase + Sized {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(&self) {}
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn out_value_1(&self) -> multiversx_sc::types::ManagedBuffer<Self::Api> {
        let s = SerdeStruct1 {
            value1: 10,
            value2: 20,
        };
        multiversx_sc::serde::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn out_value_2(&self) -> multiversx_sc::types::ManagedBuffer<Self::Api> {
        let s = ManagedSerdeStruct::<Self::Api> {
            mb: "abc".into(),
        };
        multiversx_sc::serde::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn inc_serde_1(
        &self,
        json: multiversx_sc::types::ManagedBuffer<Self::Api>,
    ) -> multiversx_sc::types::ManagedBuffer<Self::Api> {
        let mut buf = [0u8; BUFFER_SIZE];
        let slice = json.load_to_byte_array(&mut buf);
        let (mut s, _) = serde_json_core::from_slice::<SerdeStruct1>(slice)
            .unwrap_or_else(|_| multiversx_sc::contract_base::ErrorHelper::<
                Self::Api,
            >::signal_error_with_message("deserialization failed"));
        s.value1 += 1;
        s.value2 += 1;
        multiversx_sc::serde::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }
}
pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}
impl<C> SerdeFeatures for C
where
    C: AutoImpl,
{}
impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{}
pub trait EndpointWrappers: multiversx_sc::contract_base::ContractBase + SerdeFeatures {
    #[inline]
    fn call_init(&mut self) {
        <Self::Api as multiversx_sc::api::VMApi>::init_static();
        multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
        let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
        self.init();
    }
    #[inline]
    fn call_out_value_1(&mut self) {
        <Self::Api as multiversx_sc::api::VMApi>::init_static();
        multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
        let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
        let result = self.out_value_1();
        multiversx_sc::io::finish_multi::<Self::Api, _>(&result);
    }
    #[inline]
    fn call_out_value_2(&mut self) {
        <Self::Api as multiversx_sc::api::VMApi>::init_static();
        multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
        let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
        let result = self.out_value_2();
        multiversx_sc::io::finish_multi::<Self::Api, _>(&result);
    }
    #[inline]
    fn call_inc_serde_1(&mut self) {
        <Self::Api as multiversx_sc::api::VMApi>::init_static();
        multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
        let (json, ()) = multiversx_sc::io::load_endpoint_args::<
            Self::Api,
            (multiversx_sc::types::ManagedBuffer<Self::Api>, ()),
        >(("json", ()));
        let result = self.inc_serde_1(json);
        multiversx_sc::io::finish_multi::<Self::Api, _>(&result);
    }
    fn call(&mut self, fn_name: &str) -> bool {
        match fn_name {
            "callBack" => {
                self::EndpointWrappers::callback(self);
                true
            }
            "init" if <Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() => {
                multiversx_sc::external_view_contract::external_view_contract_constructor::<
                    Self::Api,
                >();
                true
            }
            "init" if !<Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() => {
                self.call_init();
                true
            }
            "out_value_1" => {
                self.call_out_value_1();
                true
            }
            "out_value_2" => {
                self.call_out_value_2();
                true
            }
            "inc_serde_1" => {
                self.call_inc_serde_1();
                true
            }
            other => false,
        }
    }
    fn callback_selector(
        &mut self,
        ___cb_closure___: &multiversx_sc::types::CallbackClosureForDeser<Self::Api>,
    ) -> multiversx_sc::types::CallbackSelectorResult {
        multiversx_sc::types::CallbackSelectorResult::NotProcessed
    }
    fn callback(&mut self) {}
}
impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{}
pub struct AbiProvider {}
impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
    type Api = multiversx_sc::api::uncallable::UncallableApi;
    fn abi() -> multiversx_sc::abi::ContractAbi {
        let mut contract_abi = multiversx_sc::abi::ContractAbi::new(
            multiversx_sc::abi::BuildInfoAbi {
                contract_crate: multiversx_sc::abi::ContractCrateBuildAbi {
                    name: "serde-features",
                    version: "0.0.0",
                    git_version: "",
                },
                framework: multiversx_sc::abi::FrameworkBuildAbi::create(),
            },
            &[],
            "SerdeFeatures",
            false,
        );
        let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
            "init",
            "init",
            multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
            multiversx_sc::abi::EndpointTypeAbi::Init,
        );
        contract_abi.constructors.push(endpoint_abi);
        let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
            "out_value_1",
            "out_value_1",
            multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
            multiversx_sc::abi::EndpointTypeAbi::Endpoint,
        );
        endpoint_abi.add_output::<multiversx_sc::types::ManagedBuffer<Self::Api>>(&[]);
        contract_abi
            .add_type_descriptions::<multiversx_sc::types::ManagedBuffer<Self::Api>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
            "out_value_2",
            "out_value_2",
            multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
            multiversx_sc::abi::EndpointTypeAbi::Endpoint,
        );
        endpoint_abi.add_output::<multiversx_sc::types::ManagedBuffer<Self::Api>>(&[]);
        contract_abi
            .add_type_descriptions::<multiversx_sc::types::ManagedBuffer<Self::Api>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
            "inc_serde_1",
            "inc_serde_1",
            multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
            multiversx_sc::abi::EndpointTypeAbi::Endpoint,
        );
        endpoint_abi.add_input::<multiversx_sc::types::ManagedBuffer<Self::Api>>("json");
        contract_abi
            .add_type_descriptions::<multiversx_sc::types::ManagedBuffer<Self::Api>>();
        endpoint_abi.add_output::<multiversx_sc::types::ManagedBuffer<Self::Api>>(&[]);
        contract_abi
            .add_type_descriptions::<multiversx_sc::types::ManagedBuffer<Self::Api>>();
        contract_abi.endpoints.push(endpoint_abi);
        contract_abi
    }
}
pub struct ContractObj<A>(
    multiversx_sc::contract_base::UniversalContractObj<A>,
)
where
    A: multiversx_sc::api::VMApi;
impl<A> multiversx_sc::contract_base::ContractBase for ContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{
    type Api = A;
}
impl<A> AutoImpl for ContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{}
impl<A> EndpointWrappers for ContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{}
impl<A> multiversx_sc::contract_base::CallableContract for ContractObj<A>
where
    A: multiversx_sc::api::VMApi + Send + Sync,
{
    fn call(&self, fn_name: &str) -> bool {
        let mut obj = multiversx_sc::contract_base::UniversalContractObj::<A>::new();
        EndpointWrappers::call(&mut obj, fn_name)
    }
}
pub fn contract_obj<A>() -> ContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{
    ContractObj::<A>(multiversx_sc::contract_base::UniversalContractObj::<A>::new())
}
pub struct ContractBuilder;
impl multiversx_sc::contract_base::CallableContractBuilder for self::ContractBuilder {
    fn new_contract_obj<A: multiversx_sc::api::VMApi + Send + Sync>(
        &self,
    ) -> multiversx_sc::types::heap::Box<
        dyn multiversx_sc::contract_base::CallableContract,
    > {
        multiversx_sc::types::heap::Box::new(self::contract_obj::<A>())
    }
}
#[allow(non_snake_case)]
pub mod __wasm__endpoints__ {
    use super::EndpointWrappers;
    pub fn init<A>()
    where
        A: multiversx_sc::api::VMApi,
    {
        super::EndpointWrappers::call_init(
            &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
        );
    }
    pub fn out_value_1<A>()
    where
        A: multiversx_sc::api::VMApi,
    {
        super::EndpointWrappers::call_out_value_1(
            &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
        );
    }
    pub fn out_value_2<A>()
    where
        A: multiversx_sc::api::VMApi,
    {
        super::EndpointWrappers::call_out_value_2(
            &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
        );
    }
    pub fn inc_serde_1<A>()
    where
        A: multiversx_sc::api::VMApi,
    {
        super::EndpointWrappers::call_inc_serde_1(
            &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
        );
    }
    pub fn callBack<A>()
    where
        A: multiversx_sc::api::VMApi,
    {
        super::EndpointWrappers::callback(
            &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
        );
    }
}
pub trait ProxyTrait: multiversx_sc::contract_base::ProxyObjBase + Sized {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(
        &mut self,
    ) -> multiversx_sc::types::Tx<
        multiversx_sc::types::TxScEnv<Self::Api>,
        (),
        Self::To,
        (),
        (),
        multiversx_sc::types::DeployCall<multiversx_sc::types::TxScEnv<Self::Api>, ()>,
        multiversx_sc::types::OriginalResultMarker<()>,
    > {
        multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
            .raw_deploy()
            .original_result()
            .to(self.extract_proxy_to())
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn out_value_1(
        &mut self,
    ) -> multiversx_sc::types::Tx<
        multiversx_sc::types::TxScEnv<Self::Api>,
        (),
        Self::To,
        (),
        (),
        multiversx_sc::types::FunctionCall<Self::Api>,
        multiversx_sc::types::OriginalResultMarker<
            multiversx_sc::types::ManagedBuffer<Self::Api>,
        >,
    > {
        multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
            .to(self.extract_proxy_to())
            .original_result()
            .raw_call("out_value_1")
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn out_value_2(
        &mut self,
    ) -> multiversx_sc::types::Tx<
        multiversx_sc::types::TxScEnv<Self::Api>,
        (),
        Self::To,
        (),
        (),
        multiversx_sc::types::FunctionCall<Self::Api>,
        multiversx_sc::types::OriginalResultMarker<
            multiversx_sc::types::ManagedBuffer<Self::Api>,
        >,
    > {
        multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
            .to(self.extract_proxy_to())
            .original_result()
            .raw_call("out_value_2")
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn inc_serde_1<
        Arg0: multiversx_sc::types::ProxyArg<
                multiversx_sc::types::ManagedBuffer<Self::Api>,
            >,
    >(
        &mut self,
        json: Arg0,
    ) -> multiversx_sc::types::Tx<
        multiversx_sc::types::TxScEnv<Self::Api>,
        (),
        Self::To,
        (),
        (),
        multiversx_sc::types::FunctionCall<Self::Api>,
        multiversx_sc::types::OriginalResultMarker<
            multiversx_sc::types::ManagedBuffer<Self::Api>,
        >,
    > {
        multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
            .to(self.extract_proxy_to())
            .original_result()
            .raw_call("inc_serde_1")
            .argument(&json)
    }
}
pub struct Proxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{
    _phantom: core::marker::PhantomData<A>,
}
impl<A> multiversx_sc::contract_base::ProxyObjBase for Proxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{
    type Api = A;
    type To = ();
    fn extract_opt_address(
        &mut self,
    ) -> multiversx_sc::types::ManagedOption<
        Self::Api,
        multiversx_sc::types::ManagedAddress<Self::Api>,
    > {
        multiversx_sc::types::ManagedOption::none()
    }
    fn extract_address(&mut self) -> multiversx_sc::types::ManagedAddress<Self::Api> {
        multiversx_sc::api::ErrorApiImpl::signal_error(
            &<A as multiversx_sc::api::ErrorApi>::error_api_impl(),
            multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET.as_bytes(),
        )
    }
    fn extract_proxy_to(&mut self) -> Self::To {}
}
impl<A> multiversx_sc::contract_base::ProxyObjNew for Proxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{
    type ProxyTo = ProxyTo<A>;
    fn new_proxy_obj() -> Self {
        Proxy {
            _phantom: core::marker::PhantomData,
        }
    }
    fn contract(
        mut self,
        address: multiversx_sc::types::ManagedAddress<Self::Api>,
    ) -> Self::ProxyTo {
        ProxyTo {
            address: multiversx_sc::types::ManagedOption::some(address),
        }
    }
}
pub struct ProxyTo<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{
    pub address: multiversx_sc::types::ManagedOption<
        A,
        multiversx_sc::types::ManagedAddress<A>,
    >,
}
impl<A> multiversx_sc::contract_base::ProxyObjBase for ProxyTo<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{
    type Api = A;
    type To = multiversx_sc::types::ManagedAddress<A>;
    fn extract_opt_address(
        &mut self,
    ) -> multiversx_sc::types::ManagedOption<
        Self::Api,
        multiversx_sc::types::ManagedAddress<Self::Api>,
    > {
        core::mem::replace(
            &mut self.address,
            multiversx_sc::types::ManagedOption::none(),
        )
    }
    fn extract_address(&mut self) -> multiversx_sc::types::ManagedAddress<Self::Api> {
        let address = core::mem::replace(
            &mut self.address,
            multiversx_sc::types::ManagedOption::none(),
        );
        address.unwrap_or_sc_panic(multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
    }
    fn extract_proxy_to(&mut self) -> Self::To {
        self.extract_address()
    }
}
impl<A> ProxyTrait for Proxy<A>
where
    A: multiversx_sc::api::VMApi,
{}
impl<A> ProxyTrait for ProxyTo<A>
where
    A: multiversx_sc::api::VMApi,
{}
