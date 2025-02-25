#![feature(prelude_import)]
#![no_std]
use core::prelude::rust_2021::*;
#[macro_use]
mod file {
    #![feature(prelude_import)]
    #![no_std]
    extern crate core;
    use core::prelude::rust_2021::*;
    #[macro_use]
    mod file {
        #![feature(prelude_import)]
        #![no_std]
        use core::prelude::rust_2021::*;
        #[macro_use]
        mod file {
            #![feature(prelude_import)]
            #![no_std]
            #[macro_use]
            use bitflags::bitflags;
            pub struct Permission(<Permission as ::bitflags::__private::PublicFlags>::Internal);
            #[automatically_derived]
            impl ::core::clone::Clone for Permission {
                #[inline]
                fn clone(&self) -> Permission {
                    Permission(::core::clone::Clone::clone(&self.0))
                }
            }
            impl Permission {
                #[allow(deprecated, non_upper_case_globals)]
                pub const NONE: Self = Self::from_bits_retain(0);
                #[allow(deprecated, non_upper_case_globals)]
                pub const OWNER: Self = Self::from_bits_retain(1);
                #[allow(deprecated, non_upper_case_globals)]
                pub const ADMIN: Self = Self::from_bits_retain(2);
                #[allow(deprecated, non_upper_case_globals)]
                pub const PAUSE: Self = Self::from_bits_retain(4);
            }
            impl ::bitflags::Flags for Permission {
                const FLAGS: &'static [::bitflags::Flag<Permission>] = &[
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("NONE", Permission::NONE)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("OWNER", Permission::OWNER)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("ADMIN", Permission::ADMIN)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("PAUSE", Permission::PAUSE)
                    },
                ];
                type Bits = u32;
                fn bits(&self) -> u32 {
                    Permission::bits(self)
                }
                fn from_bits_retain(bits: u32) -> Permission {
                    Permission::from_bits_retain(bits)
                }
            }
            #[allow(
                dead_code,
                deprecated,
                unused_doc_comments,
                unused_attributes,
                unused_mut,
                unused_imports,
                non_upper_case_globals,
                clippy::assign_op_pattern,
                clippy::indexing_slicing,
                clippy::same_name_method,
                clippy::iter_without_into_iter
            )]
            const _: () = {
                #[repr(transparent)]
                pub struct InternalBitFlags(u32);
                #[automatically_derived]
                impl ::core::clone::Clone for InternalBitFlags {
                    #[inline]
                    fn clone(&self) -> InternalBitFlags {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for InternalBitFlags {
                    #[inline]
                    fn eq(&self, other: &InternalBitFlags) -> bool {
                        self.0 == other.0
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for InternalBitFlags {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::PartialOrd for InternalBitFlags {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &InternalBitFlags,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Ord for InternalBitFlags {
                    #[inline]
                    fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for InternalBitFlags {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        ::core::hash::Hash::hash(&self.0, state)
                    }
                }
                impl ::bitflags::__private::PublicFlags for Permission {
                    type Primitive = u32;
                    type Internal = InternalBitFlags;
                }
                impl ::bitflags::__private::core::default::Default for InternalBitFlags {
                    #[inline]
                    fn default() -> Self {
                        InternalBitFlags::empty()
                    }
                }
                impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        if self.is_empty() {
                            f.write_fmt(format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY))
                        } else {
                            ::bitflags::__private::core::fmt::Display::fmt(self, f)
                        }
                    }
                }
                impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        ::bitflags::parser::to_writer(&Permission(*self), f)
                    }
                }
                impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
                    type Err = ::bitflags::parser::ParseError;
                    fn from_str(
                        s: &str,
                    ) -> ::bitflags::__private::core::result::Result<Self, Self::Err>
                    {
                        ::bitflags::parser::from_str::<Permission>(s).map(|flags| flags.0)
                    }
                }
                impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
                    fn as_ref(&self) -> &u32 {
                        &self.0
                    }
                }
                impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
                    fn from(bits: u32) -> Self {
                        Self::from_bits_retain(bits)
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl InternalBitFlags {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        {
                            Self(<u32 as ::bitflags::Bits>::EMPTY)
                        }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        {
                            let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                            let mut i = 0;
                            {
                                {
                                    let flag =
                                        <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag =
                                        <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag =
                                        <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag =
                                        <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            let _ = i;
                            Self::from_bits_retain(truncated)
                        }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u32 {
                        let f = self;
                        {
                            f.0
                        }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u32,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            let truncated = Self::from_bits_truncate(bits).0;
                            if truncated == bits {
                                ::bitflags::__private::core::option::Option::Some(Self(bits))
                            } else {
                                ::bitflags::__private::core::option::Option::None
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u32) -> Self {
                        let bits = bits;
                        {
                            Self(bits & Self::all().bits())
                        }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u32) -> Self {
                        let bits = bits;
                        {
                            Self(bits)
                        }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            {
                                if name == "NONE" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(Permission::NONE.bits()),
                                    );
                                }
                            };
                            {
                                if name == "OWNER" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(Permission::OWNER.bits()),
                                    );
                                }
                            };
                            {
                                if name == "ADMIN" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(Permission::ADMIN.bits()),
                                    );
                                }
                            };
                            {
                                if name == "PAUSE" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(Permission::PAUSE.bits()),
                                    );
                                }
                            };
                            let _ = name;
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        {
                            f.bits() == <u32 as ::bitflags::Bits>::EMPTY
                        }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        {
                            Self::all().bits() | f.bits() == f.bits()
                        }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        {
                            f.bits() & other.bits() != <u32 as ::bitflags::Bits>::EMPTY
                        }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        {
                            f.bits() & other.bits() == other.bits()
                        }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).union(other);
                        }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).difference(other);
                        }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).symmetric_difference(other);
                        }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        {
                            if value {
                                f.insert(other);
                            } else {
                                f.remove(other);
                            }
                        }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self::from_bits_retain(f.bits() & other.bits())
                        }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self::from_bits_retain(f.bits() | other.bits())
                        }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self::from_bits_retain(f.bits() & !other.bits())
                        }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self::from_bits_retain(f.bits() ^ other.bits())
                        }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        {
                            Self::from_bits_truncate(!f.bits())
                        }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: InternalBitFlags) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<InternalBitFlags> for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                        &mut self,
                        iterator: T,
                    ) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags> for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(
                        iterator: T,
                    ) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl InternalBitFlags {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <Permission as ::bitflags::Flags>::FLAGS,
                            Permission::from_bits_retain(self.bits()),
                            Permission::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <Permission as ::bitflags::Flags>::FLAGS,
                            Permission::from_bits_retain(self.bits()),
                            Permission::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
                    type Item = Permission;
                    type IntoIter = ::bitflags::iter::Iter<Permission>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
                impl InternalBitFlags {
                    /// Returns a mutable reference to the raw value of the flags currently stored.
                    #[inline]
                    pub fn bits_mut(&mut self) -> &mut u32 {
                        &mut self.0
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl Permission {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        {
                            Self(InternalBitFlags::empty())
                        }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        {
                            Self(InternalBitFlags::all())
                        }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u32 {
                        let f = self;
                        {
                            f.0.bits()
                        }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u32,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            match InternalBitFlags::from_bits(bits) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(Self(bits))
                                },
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                },
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u32) -> Self {
                        let bits = bits;
                        {
                            Self(InternalBitFlags::from_bits_truncate(bits))
                        }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u32) -> Self {
                        let bits = bits;
                        {
                            Self(InternalBitFlags::from_bits_retain(bits))
                        }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            match InternalBitFlags::from_name(name) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(Self(bits))
                                },
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                },
                            }
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        {
                            f.0.is_empty()
                        }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        {
                            f.0.is_all()
                        }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        {
                            f.0.intersects(other.0)
                        }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        {
                            f.0.contains(other.0)
                        }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            f.0.insert(other.0)
                        }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            f.0.remove(other.0)
                        }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            f.0.toggle(other.0)
                        }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        {
                            f.0.set(other.0, value)
                        }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self(f.0.intersection(other.0))
                        }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self(f.0.union(other.0))
                        }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self(f.0.difference(other.0))
                        }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        {
                            Self(f.0.symmetric_difference(other.0))
                        }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        {
                            Self(f.0.complement())
                        }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for Permission {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for Permission {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for Permission {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for Permission {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for Permission {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: Permission) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for Permission {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for Permission {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign for Permission {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for Permission {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign for Permission {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for Permission {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for Permission {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for Permission {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<Permission> for Permission {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                        &mut self,
                        iterator: T,
                    ) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<Permission> for Permission {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(
                        iterator: T,
                    ) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl Permission {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <Permission as ::bitflags::Flags>::FLAGS,
                            Permission::from_bits_retain(self.bits()),
                            Permission::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <Permission as ::bitflags::Flags>::FLAGS,
                            Permission::from_bits_retain(self.bits()),
                            Permission::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator for Permission {
                    type Item = Permission;
                    type IntoIter = ::bitflags::iter::Iter<Permission>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
            };
            /// An empty contract. To be used as a template when starting a new contract from scratch.
            pub trait EmptyContract: multiversx_sc::contract_base::ContractBase + Sized {
                #[allow(clippy::too_many_arguments)]
                #[allow(clippy::type_complexity)]
                fn init(&self) {}
                #[allow(clippy::too_many_arguments)]
                #[allow(clippy::type_complexity)]
                fn upgrade(&self) {}
            }
            pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}
            impl<C> EmptyContract for C where C: AutoImpl {}
            impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
                A: multiversx_sc::api::VMApi
            {
            }
            pub trait EndpointWrappers:
                multiversx_sc::contract_base::ContractBase + EmptyContract
            {
                #[inline]
                fn call_init(&mut self) {
                    <Self::Api as multiversx_sc::api::VMApi>::init_static();
                    multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
                    let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
                    self.init();
                }
                #[inline]
                fn call_upgrade(&mut self) {
                    <Self::Api as multiversx_sc::api::VMApi>::init_static();
                    multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
                    let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
                    self.upgrade();
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
                        "upgrade" => {
                            self.call_upgrade();
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
            impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
                A: multiversx_sc::api::VMApi
            {
            }
            pub struct AbiProvider {}
            impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
                type Api = multiversx_sc::api::uncallable::UncallableApi;
                fn abi() -> multiversx_sc::abi::ContractAbi {
                    let mut contract_abi = multiversx_sc::abi::ContractAbi::new(
                        multiversx_sc::abi::BuildInfoAbi {
                            contract_crate: multiversx_sc::abi::ContractCrateBuildAbi {
                                name: "empty",
                                version: "0.0.0",
                                git_version: "",
                            },
                            framework: multiversx_sc::abi::FrameworkBuildAbi::create(),
                        },
                        &[
                            "An empty contract. To be used as a template when starting a new contract from scratch.",
                        ],
                        "EmptyContract",
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
                        "upgrade",
                        "upgrade",
                        multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                        multiversx_sc::abi::EndpointTypeAbi::Upgrade,
                    );
                    contract_abi.upgrade_constructors.push(endpoint_abi);
                    contract_abi
                }
            }
            pub struct ContractObj<A>(multiversx_sc::contract_base::UniversalContractObj<A>)
            where
                A: multiversx_sc::api::VMApi;
            impl<A> multiversx_sc::contract_base::ContractBase for ContractObj<A>
            where
                A: multiversx_sc::api::VMApi,
            {
                type Api = A;
            }
            impl<A> AutoImpl for ContractObj<A> where A: multiversx_sc::api::VMApi {}
            impl<A> EndpointWrappers for ContractObj<A> where A: multiversx_sc::api::VMApi {}
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
                pub fn upgrade<A>()
                where
                    A: multiversx_sc::api::VMApi,
                {
                    super::EndpointWrappers::call_upgrade(
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
                fn upgrade(
                    &mut self,
                ) -> multiversx_sc::types::Tx<
                    multiversx_sc::types::TxScEnv<Self::Api>,
                    (),
                    Self::To,
                    (),
                    (),
                    multiversx_sc::types::FunctionCall<Self::Api>,
                    multiversx_sc::types::OriginalResultMarker<()>,
                > {
                    multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                        .to(self.extract_proxy_to())
                        .original_result()
                        .raw_call("upgrade")
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
                pub address:
                    multiversx_sc::types::ManagedOption<A, multiversx_sc::types::ManagedAddress<A>>,
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
            impl<A> ProxyTrait for Proxy<A> where A: multiversx_sc::api::VMApi {}
            impl<A> ProxyTrait for ProxyTo<A> where A: multiversx_sc::api::VMApi {}
        }
        use bitflags::bitflags;
        use multiversx_sc::derive::type_abi;
        pub struct Permission(<Permission as ::bitflags::__private::PublicFlags>::Internal);
        #[automatically_derived]
        impl ::core::clone::Clone for Permission {
            #[inline]
            fn clone(&self) -> Permission {
                Permission(::core::clone::Clone::clone(&self.0))
            }
        }
        impl multiversx_sc::abi::TypeAbiFrom<Self> for Permission {}
        impl multiversx_sc::abi::TypeAbiFrom<&Self> for Permission {}
        impl multiversx_sc::abi::TypeAbi for Permission {
            type Unmanaged = Self;
            fn type_name() -> multiversx_sc::abi::TypeName {
                "Permission".into()
            }
            fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
                accumulator: &mut TDC,
            ) {
                let type_names = Self::type_names();
                if !accumulator.contains_type(&type_names.abi) {
                    accumulator.reserve_type_name(type_names.clone());
                    let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
                    field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                        &[],
                        "0",
                        <<Permission as ::bitflags::__private::PublicFlags>::Internal>::type_names(
                        ),
                    ));
                    <<Permission as ::bitflags::__private::PublicFlags>::Internal>::provide_type_descriptions(
                        accumulator,
                    );
                    accumulator.insert(
                        type_names.clone(),
                        multiversx_sc::abi::TypeDescription::new(
                            &[],
                            type_names,
                            multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                            &["Clone"],
                        ),
                    );
                }
            }
        }
        impl Permission {
            #[allow(deprecated, non_upper_case_globals)]
            pub const NONE: Self = Self::from_bits_retain(0);
            #[allow(deprecated, non_upper_case_globals)]
            pub const OWNER: Self = Self::from_bits_retain(1);
            #[allow(deprecated, non_upper_case_globals)]
            pub const ADMIN: Self = Self::from_bits_retain(2);
            #[allow(deprecated, non_upper_case_globals)]
            pub const PAUSE: Self = Self::from_bits_retain(4);
        }
        impl ::bitflags::Flags for Permission {
            const FLAGS: &'static [::bitflags::Flag<Permission>] = &[
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new("NONE", Permission::NONE)
                },
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new("OWNER", Permission::OWNER)
                },
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new("ADMIN", Permission::ADMIN)
                },
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new("PAUSE", Permission::PAUSE)
                },
            ];
            type Bits = u32;
            fn bits(&self) -> u32 {
                Permission::bits(self)
            }
            fn from_bits_retain(bits: u32) -> Permission {
                Permission::from_bits_retain(bits)
            }
        }
        #[allow(
            dead_code,
            deprecated,
            unused_doc_comments,
            unused_attributes,
            unused_mut,
            unused_imports,
            non_upper_case_globals,
            clippy::assign_op_pattern,
            clippy::indexing_slicing,
            clippy::same_name_method,
            clippy::iter_without_into_iter
        )]
        const _: () = {
            #[repr(transparent)]
            pub struct InternalBitFlags(u32);
            #[automatically_derived]
            impl ::core::clone::Clone for InternalBitFlags {
                #[inline]
                fn clone(&self) -> InternalBitFlags {
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for InternalBitFlags {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for InternalBitFlags {
                #[inline]
                fn eq(&self, other: &InternalBitFlags) -> bool {
                    self.0 == other.0
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for InternalBitFlags {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for InternalBitFlags {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &InternalBitFlags,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for InternalBitFlags {
                #[inline]
                fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                    ::core::cmp::Ord::cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for InternalBitFlags {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.0, state)
                }
            }
            impl ::bitflags::__private::PublicFlags for Permission {
                type Primitive = u32;
                type Internal = InternalBitFlags;
            }
            impl ::bitflags::__private::core::default::Default for InternalBitFlags {
                #[inline]
                fn default() -> Self {
                    InternalBitFlags::empty()
                }
            }
            impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                ) -> ::bitflags::__private::core::fmt::Result {
                    if self.is_empty() {
                        f.write_fmt(format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY))
                    } else {
                        ::bitflags::__private::core::fmt::Display::fmt(self, f)
                    }
                }
            }
            impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                ) -> ::bitflags::__private::core::fmt::Result {
                    ::bitflags::parser::to_writer(&Permission(*self), f)
                }
            }
            impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
                type Err = ::bitflags::parser::ParseError;
                fn from_str(
                    s: &str,
                ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                    ::bitflags::parser::from_str::<Permission>(s).map(|flags| flags.0)
                }
            }
            impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
                fn as_ref(&self) -> &u32 {
                    &self.0
                }
            }
            impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
                fn from(bits: u32) -> Self {
                    Self::from_bits_retain(bits)
                }
            }
            #[allow(dead_code, deprecated, unused_attributes)]
            impl InternalBitFlags {
                /// Get a flags value with all bits unset.
                #[inline]
                pub const fn empty() -> Self {
                    {
                        Self(<u32 as ::bitflags::Bits>::EMPTY)
                    }
                }
                /// Get a flags value with all known bits set.
                #[inline]
                pub const fn all() -> Self {
                    {
                        let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                        let mut i = 0;
                        {
                            {
                                let flag =
                                    <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                truncated = truncated | flag;
                                i += 1;
                            }
                        };
                        {
                            {
                                let flag =
                                    <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                truncated = truncated | flag;
                                i += 1;
                            }
                        };
                        {
                            {
                                let flag =
                                    <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                truncated = truncated | flag;
                                i += 1;
                            }
                        };
                        {
                            {
                                let flag =
                                    <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                                truncated = truncated | flag;
                                i += 1;
                            }
                        };
                        let _ = i;
                        Self::from_bits_retain(truncated)
                    }
                }
                /// Get the underlying bits value.
                ///
                /// The returned value is exactly the bits set in this flags value.
                #[inline]
                pub const fn bits(&self) -> u32 {
                    let f = self;
                    {
                        f.0
                    }
                }
                /// Convert from a bits value.
                ///
                /// This method will return `None` if any unknown bits are set.
                #[inline]
                pub const fn from_bits(
                    bits: u32,
                ) -> ::bitflags::__private::core::option::Option<Self> {
                    let bits = bits;
                    {
                        let truncated = Self::from_bits_truncate(bits).0;
                        if truncated == bits {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        } else {
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                }
                /// Convert from a bits value, unsetting any unknown bits.
                #[inline]
                pub const fn from_bits_truncate(bits: u32) -> Self {
                    let bits = bits;
                    {
                        Self(bits & Self::all().bits())
                    }
                }
                /// Convert from a bits value exactly.
                #[inline]
                pub const fn from_bits_retain(bits: u32) -> Self {
                    let bits = bits;
                    {
                        Self(bits)
                    }
                }
                /// Get a flags value with the bits of a flag with the given name set.
                ///
                /// This method will return `None` if `name` is empty or doesn't
                /// correspond to any named flag.
                #[inline]
                pub fn from_name(name: &str) -> ::bitflags::__private::core::option::Option<Self> {
                    let name = name;
                    {
                        {
                            if name == "NONE" {
                                return ::bitflags::__private::core::option::Option::Some(Self(
                                    Permission::NONE.bits(),
                                ));
                            }
                        };
                        {
                            if name == "OWNER" {
                                return ::bitflags::__private::core::option::Option::Some(Self(
                                    Permission::OWNER.bits(),
                                ));
                            }
                        };
                        {
                            if name == "ADMIN" {
                                return ::bitflags::__private::core::option::Option::Some(Self(
                                    Permission::ADMIN.bits(),
                                ));
                            }
                        };
                        {
                            if name == "PAUSE" {
                                return ::bitflags::__private::core::option::Option::Some(Self(
                                    Permission::PAUSE.bits(),
                                ));
                            }
                        };
                        let _ = name;
                        ::bitflags::__private::core::option::Option::None
                    }
                }
                /// Whether all bits in this flags value are unset.
                #[inline]
                pub const fn is_empty(&self) -> bool {
                    let f = self;
                    {
                        f.bits() == <u32 as ::bitflags::Bits>::EMPTY
                    }
                }
                /// Whether all known bits in this flags value are set.
                #[inline]
                pub const fn is_all(&self) -> bool {
                    let f = self;
                    {
                        Self::all().bits() | f.bits() == f.bits()
                    }
                }
                /// Whether any set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn intersects(&self, other: Self) -> bool {
                    let f = self;
                    let other = other;
                    {
                        f.bits() & other.bits() != <u32 as ::bitflags::Bits>::EMPTY
                    }
                }
                /// Whether all set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn contains(&self, other: Self) -> bool {
                    let f = self;
                    let other = other;
                    {
                        f.bits() & other.bits() == other.bits()
                    }
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                pub fn insert(&mut self, other: Self) {
                    let f = self;
                    let other = other;
                    {
                        *f = Self::from_bits_retain(f.bits()).union(other);
                    }
                }
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `remove` won't truncate `other`, but the `!` operator will.
                #[inline]
                pub fn remove(&mut self, other: Self) {
                    let f = self;
                    let other = other;
                    {
                        *f = Self::from_bits_retain(f.bits()).difference(other);
                    }
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                pub fn toggle(&mut self, other: Self) {
                    let f = self;
                    let other = other;
                    {
                        *f = Self::from_bits_retain(f.bits()).symmetric_difference(other);
                    }
                }
                /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                #[inline]
                pub fn set(&mut self, other: Self, value: bool) {
                    let f = self;
                    let other = other;
                    let value = value;
                    {
                        if value {
                            f.insert(other);
                        } else {
                            f.remove(other);
                        }
                    }
                }
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn intersection(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self::from_bits_retain(f.bits() & other.bits())
                    }
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn union(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self::from_bits_retain(f.bits() | other.bits())
                    }
                }
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                #[must_use]
                pub const fn difference(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self::from_bits_retain(f.bits() & !other.bits())
                    }
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn symmetric_difference(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self::from_bits_retain(f.bits() ^ other.bits())
                    }
                }
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                #[must_use]
                pub const fn complement(self) -> Self {
                    let f = self;
                    {
                        Self::from_bits_truncate(!f.bits())
                    }
                }
            }
            impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
                type Output = Self;
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor(self, other: InternalBitFlags) -> Self {
                    self.union(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor_assign(&mut self, other: Self) {
                    self.insert(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
                type Output = Self;
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor(self, other: Self) -> Self {
                    self.symmetric_difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor_assign(&mut self, other: Self) {
                    self.toggle(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
                type Output = Self;
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand(self, other: Self) -> Self {
                    self.intersection(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand_assign(&mut self, other: Self) {
                    *self = Self::from_bits_retain(self.bits()).intersection(other);
                }
            }
            impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
                type Output = Self;
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub(self, other: Self) -> Self {
                    self.difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub_assign(&mut self, other: Self) {
                    self.remove(other);
                }
            }
            impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
                type Output = Self;
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                fn not(self) -> Self {
                    self.complement()
                }
            }
            impl ::bitflags::__private::core::iter::Extend<InternalBitFlags> for InternalBitFlags {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                    &mut self,
                    iterator: T,
                ) {
                    for item in iterator {
                        self.insert(item)
                    }
                }
            }
            impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags> for InternalBitFlags {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn from_iter<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                    iterator: T,
                ) -> Self {
                    use ::bitflags::__private::core::iter::Extend;
                    let mut result = Self::empty();
                    result.extend(iterator);
                    result
                }
            }
            impl InternalBitFlags {
                /// Yield a set of contained flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                /// will be yielded together as a final flags value.
                #[inline]
                pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
                    ::bitflags::iter::Iter::__private_const_new(
                        <Permission as ::bitflags::Flags>::FLAGS,
                        Permission::from_bits_retain(self.bits()),
                        Permission::from_bits_retain(self.bits()),
                    )
                }
                /// Yield a set of contained named flags values.
                ///
                /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                #[inline]
                pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
                    ::bitflags::iter::IterNames::__private_const_new(
                        <Permission as ::bitflags::Flags>::FLAGS,
                        Permission::from_bits_retain(self.bits()),
                        Permission::from_bits_retain(self.bits()),
                    )
                }
            }
            impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
                type Item = Permission;
                type IntoIter = ::bitflags::iter::Iter<Permission>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }
            impl InternalBitFlags {
                /// Returns a mutable reference to the raw value of the flags currently stored.
                #[inline]
                pub fn bits_mut(&mut self) -> &mut u32 {
                    &mut self.0
                }
            }
            #[allow(dead_code, deprecated, unused_attributes)]
            impl Permission {
                /// Get a flags value with all bits unset.
                #[inline]
                pub const fn empty() -> Self {
                    {
                        Self(InternalBitFlags::empty())
                    }
                }
                /// Get a flags value with all known bits set.
                #[inline]
                pub const fn all() -> Self {
                    {
                        Self(InternalBitFlags::all())
                    }
                }
                /// Get the underlying bits value.
                ///
                /// The returned value is exactly the bits set in this flags value.
                #[inline]
                pub const fn bits(&self) -> u32 {
                    let f = self;
                    {
                        f.0.bits()
                    }
                }
                /// Convert from a bits value.
                ///
                /// This method will return `None` if any unknown bits are set.
                #[inline]
                pub const fn from_bits(
                    bits: u32,
                ) -> ::bitflags::__private::core::option::Option<Self> {
                    let bits = bits;
                    {
                        match InternalBitFlags::from_bits(bits) {
                            ::bitflags::__private::core::option::Option::Some(bits) => {
                                ::bitflags::__private::core::option::Option::Some(Self(bits))
                            },
                            ::bitflags::__private::core::option::Option::None => {
                                ::bitflags::__private::core::option::Option::None
                            },
                        }
                    }
                }
                /// Convert from a bits value, unsetting any unknown bits.
                #[inline]
                pub const fn from_bits_truncate(bits: u32) -> Self {
                    let bits = bits;
                    {
                        Self(InternalBitFlags::from_bits_truncate(bits))
                    }
                }
                /// Convert from a bits value exactly.
                #[inline]
                pub const fn from_bits_retain(bits: u32) -> Self {
                    let bits = bits;
                    {
                        Self(InternalBitFlags::from_bits_retain(bits))
                    }
                }
                /// Get a flags value with the bits of a flag with the given name set.
                ///
                /// This method will return `None` if `name` is empty or doesn't
                /// correspond to any named flag.
                #[inline]
                pub fn from_name(name: &str) -> ::bitflags::__private::core::option::Option<Self> {
                    let name = name;
                    {
                        match InternalBitFlags::from_name(name) {
                            ::bitflags::__private::core::option::Option::Some(bits) => {
                                ::bitflags::__private::core::option::Option::Some(Self(bits))
                            },
                            ::bitflags::__private::core::option::Option::None => {
                                ::bitflags::__private::core::option::Option::None
                            },
                        }
                    }
                }
                /// Whether all bits in this flags value are unset.
                #[inline]
                pub const fn is_empty(&self) -> bool {
                    let f = self;
                    {
                        f.0.is_empty()
                    }
                }
                /// Whether all known bits in this flags value are set.
                #[inline]
                pub const fn is_all(&self) -> bool {
                    let f = self;
                    {
                        f.0.is_all()
                    }
                }
                /// Whether any set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn intersects(&self, other: Self) -> bool {
                    let f = self;
                    let other = other;
                    {
                        f.0.intersects(other.0)
                    }
                }
                /// Whether all set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn contains(&self, other: Self) -> bool {
                    let f = self;
                    let other = other;
                    {
                        f.0.contains(other.0)
                    }
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                pub fn insert(&mut self, other: Self) {
                    let f = self;
                    let other = other;
                    {
                        f.0.insert(other.0)
                    }
                }
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `remove` won't truncate `other`, but the `!` operator will.
                #[inline]
                pub fn remove(&mut self, other: Self) {
                    let f = self;
                    let other = other;
                    {
                        f.0.remove(other.0)
                    }
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                pub fn toggle(&mut self, other: Self) {
                    let f = self;
                    let other = other;
                    {
                        f.0.toggle(other.0)
                    }
                }
                /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                #[inline]
                pub fn set(&mut self, other: Self, value: bool) {
                    let f = self;
                    let other = other;
                    let value = value;
                    {
                        f.0.set(other.0, value)
                    }
                }
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn intersection(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self(f.0.intersection(other.0))
                    }
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn union(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self(f.0.union(other.0))
                    }
                }
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                #[must_use]
                pub const fn difference(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self(f.0.difference(other.0))
                    }
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn symmetric_difference(self, other: Self) -> Self {
                    let f = self;
                    let other = other;
                    {
                        Self(f.0.symmetric_difference(other.0))
                    }
                }
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                #[must_use]
                pub const fn complement(self) -> Self {
                    let f = self;
                    {
                        Self(f.0.complement())
                    }
                }
            }
            impl ::bitflags::__private::core::fmt::Binary for Permission {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::Octal for Permission {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::LowerHex for Permission {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::UpperHex for Permission {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::ops::BitOr for Permission {
                type Output = Self;
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor(self, other: Permission) -> Self {
                    self.union(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitOrAssign for Permission {
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor_assign(&mut self, other: Self) {
                    self.insert(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitXor for Permission {
                type Output = Self;
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor(self, other: Self) -> Self {
                    self.symmetric_difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitXorAssign for Permission {
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor_assign(&mut self, other: Self) {
                    self.toggle(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitAnd for Permission {
                type Output = Self;
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand(self, other: Self) -> Self {
                    self.intersection(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitAndAssign for Permission {
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand_assign(&mut self, other: Self) {
                    *self = Self::from_bits_retain(self.bits()).intersection(other);
                }
            }
            impl ::bitflags::__private::core::ops::Sub for Permission {
                type Output = Self;
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub(self, other: Self) -> Self {
                    self.difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::SubAssign for Permission {
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub_assign(&mut self, other: Self) {
                    self.remove(other);
                }
            }
            impl ::bitflags::__private::core::ops::Not for Permission {
                type Output = Self;
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                fn not(self) -> Self {
                    self.complement()
                }
            }
            impl ::bitflags::__private::core::iter::Extend<Permission> for Permission {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                    &mut self,
                    iterator: T,
                ) {
                    for item in iterator {
                        self.insert(item)
                    }
                }
            }
            impl ::bitflags::__private::core::iter::FromIterator<Permission> for Permission {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn from_iter<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                    iterator: T,
                ) -> Self {
                    use ::bitflags::__private::core::iter::Extend;
                    let mut result = Self::empty();
                    result.extend(iterator);
                    result
                }
            }
            impl Permission {
                /// Yield a set of contained flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                /// will be yielded together as a final flags value.
                #[inline]
                pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
                    ::bitflags::iter::Iter::__private_const_new(
                        <Permission as ::bitflags::Flags>::FLAGS,
                        Permission::from_bits_retain(self.bits()),
                        Permission::from_bits_retain(self.bits()),
                    )
                }
                /// Yield a set of contained named flags values.
                ///
                /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                #[inline]
                pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
                    ::bitflags::iter::IterNames::__private_const_new(
                        <Permission as ::bitflags::Flags>::FLAGS,
                        Permission::from_bits_retain(self.bits()),
                        Permission::from_bits_retain(self.bits()),
                    )
                }
            }
            impl ::bitflags::__private::core::iter::IntoIterator for Permission {
                type Item = Permission;
                type IntoIter = ::bitflags::iter::Iter<Permission>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }
        };
        /// An empty contract. To be used as a template when starting a new contract from scratch.
        pub trait EmptyContract: multiversx_sc::contract_base::ContractBase + Sized {
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn init(&self) {}
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn upgrade(&self) {}
        }
        pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}
        impl<C> EmptyContract for C where C: AutoImpl {}
        impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
            A: multiversx_sc::api::VMApi
        {
        }
        pub trait EndpointWrappers:
            multiversx_sc::contract_base::ContractBase + EmptyContract
        {
            #[inline]
            fn call_init(&mut self) {
                <Self::Api as multiversx_sc::api::VMApi>::init_static();
                multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
                let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
                self.init();
            }
            #[inline]
            fn call_upgrade(&mut self) {
                <Self::Api as multiversx_sc::api::VMApi>::init_static();
                multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
                let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
                self.upgrade();
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
                    "upgrade" => {
                        self.call_upgrade();
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
        impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
            A: multiversx_sc::api::VMApi
        {
        }
        pub struct AbiProvider {}
        impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
            type Api = multiversx_sc::api::uncallable::UncallableApi;
            fn abi() -> multiversx_sc::abi::ContractAbi {
                let mut contract_abi = multiversx_sc::abi::ContractAbi::new(
                    multiversx_sc::abi::BuildInfoAbi {
                        contract_crate: multiversx_sc::abi::ContractCrateBuildAbi {
                            name: "empty",
                            version: "0.0.0",
                            git_version: "",
                        },
                        framework: multiversx_sc::abi::FrameworkBuildAbi::create(),
                    },
                    &[
                        "An empty contract. To be used as a template when starting a new contract from scratch.",
                    ],
                    "EmptyContract",
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
                    "upgrade",
                    "upgrade",
                    multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                    multiversx_sc::abi::EndpointTypeAbi::Upgrade,
                );
                contract_abi.upgrade_constructors.push(endpoint_abi);
                contract_abi
            }
        }
        pub struct ContractObj<A>(multiversx_sc::contract_base::UniversalContractObj<A>)
        where
            A: multiversx_sc::api::VMApi;
        impl<A> multiversx_sc::contract_base::ContractBase for ContractObj<A>
        where
            A: multiversx_sc::api::VMApi,
        {
            type Api = A;
        }
        impl<A> AutoImpl for ContractObj<A> where A: multiversx_sc::api::VMApi {}
        impl<A> EndpointWrappers for ContractObj<A> where A: multiversx_sc::api::VMApi {}
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
            ) -> multiversx_sc::types::heap::Box<dyn multiversx_sc::contract_base::CallableContract>
            {
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
            pub fn upgrade<A>()
            where
                A: multiversx_sc::api::VMApi,
            {
                super::EndpointWrappers::call_upgrade(
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
            fn upgrade(
                &mut self,
            ) -> multiversx_sc::types::Tx<
                multiversx_sc::types::TxScEnv<Self::Api>,
                (),
                Self::To,
                (),
                (),
                multiversx_sc::types::FunctionCall<Self::Api>,
                multiversx_sc::types::OriginalResultMarker<()>,
            > {
                multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                    .to(self.extract_proxy_to())
                    .original_result()
                    .raw_call("upgrade")
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
            pub address:
                multiversx_sc::types::ManagedOption<A, multiversx_sc::types::ManagedAddress<A>>,
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
        impl<A> ProxyTrait for Proxy<A> where A: multiversx_sc::api::VMApi {}
        impl<A> ProxyTrait for ProxyTo<A> where A: multiversx_sc::api::VMApi {}
    }
    use bitflags::bitflags;
    use multiversx_sc::derive::type_abi;
    pub struct Permission(<Permission as ::bitflags::__private::PublicFlags>::Internal);
    #[automatically_derived]
    impl ::core::clone::Clone for Permission {
        #[inline]
        fn clone(&self) -> Permission {
            Permission(::core::clone::Clone::clone(&self.0))
        }
    }
    impl Permission {
        #[allow(deprecated, non_upper_case_globals)]
        pub const NONE: Self = Self::from_bits_retain(0);
        #[allow(deprecated, non_upper_case_globals)]
        pub const OWNER: Self = Self::from_bits_retain(1);
        #[allow(deprecated, non_upper_case_globals)]
        pub const ADMIN: Self = Self::from_bits_retain(2);
        #[allow(deprecated, non_upper_case_globals)]
        pub const PAUSE: Self = Self::from_bits_retain(4);
    }
    impl ::bitflags::Flags for Permission {
        const FLAGS: &'static [::bitflags::Flag<Permission>] = &[
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("NONE", Permission::NONE)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("OWNER", Permission::OWNER)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("ADMIN", Permission::ADMIN)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("PAUSE", Permission::PAUSE)
            },
        ];
        type Bits = u32;
        fn bits(&self) -> u32 {
            Permission::bits(self)
        }
        fn from_bits_retain(bits: u32) -> Permission {
            Permission::from_bits_retain(bits)
        }
    }
    #[allow(
        dead_code,
        deprecated,
        unused_doc_comments,
        unused_attributes,
        unused_mut,
        unused_imports,
        non_upper_case_globals,
        clippy::assign_op_pattern,
        clippy::indexing_slicing,
        clippy::same_name_method,
        clippy::iter_without_into_iter
    )]
    const _: () = {
        #[repr(transparent)]
        pub struct InternalBitFlags(u32);
        #[automatically_derived]
        impl ::core::clone::Clone for InternalBitFlags {
            #[inline]
            fn clone(&self) -> InternalBitFlags {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InternalBitFlags {
            #[inline]
            fn eq(&self, other: &InternalBitFlags) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InternalBitFlags {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InternalBitFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InternalBitFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InternalBitFlags {
            #[inline]
            fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InternalBitFlags {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl ::bitflags::__private::PublicFlags for Permission {
            type Primitive = u32;
            type Internal = InternalBitFlags;
        }
        impl ::bitflags::__private::core::default::Default for InternalBitFlags {
            #[inline]
            fn default() -> Self {
                InternalBitFlags::empty()
            }
        }
        impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                if self.is_empty() {
                    f.write_fmt(format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY))
                } else {
                    ::bitflags::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::parser::to_writer(&Permission(*self), f)
            }
        }
        impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
            type Err = ::bitflags::parser::ParseError;
            fn from_str(s: &str) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                ::bitflags::parser::from_str::<Permission>(s).map(|flags| flags.0)
            }
        }
        impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
            fn as_ref(&self) -> &u32 {
                &self.0
            }
        }
        impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
            fn from(bits: u32) -> Self {
                Self::from_bits_retain(bits)
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl InternalBitFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                {
                    Self(<u32 as ::bitflags::Bits>::EMPTY)
                }
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                {
                    let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                    let mut i = 0;
                    {
                        {
                            let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    let _ = i;
                    Self::from_bits_retain(truncated)
                }
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                let f = self;
                {
                    f.0
                }
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(bits: u32) -> ::bitflags::__private::core::option::Option<Self> {
                let bits = bits;
                {
                    let truncated = Self::from_bits_truncate(bits).0;
                    if truncated == bits {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    } else {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                let bits = bits;
                {
                    Self(bits & Self::all().bits())
                }
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                let bits = bits;
                {
                    Self(bits)
                }
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(name: &str) -> ::bitflags::__private::core::option::Option<Self> {
                let name = name;
                {
                    {
                        if name == "NONE" {
                            return ::bitflags::__private::core::option::Option::Some(Self(
                                Permission::NONE.bits(),
                            ));
                        }
                    };
                    {
                        if name == "OWNER" {
                            return ::bitflags::__private::core::option::Option::Some(Self(
                                Permission::OWNER.bits(),
                            ));
                        }
                    };
                    {
                        if name == "ADMIN" {
                            return ::bitflags::__private::core::option::Option::Some(Self(
                                Permission::ADMIN.bits(),
                            ));
                        }
                    };
                    {
                        if name == "PAUSE" {
                            return ::bitflags::__private::core::option::Option::Some(Self(
                                Permission::PAUSE.bits(),
                            ));
                        }
                    };
                    let _ = name;
                    ::bitflags::__private::core::option::Option::None
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let f = self;
                {
                    f.bits() == <u32 as ::bitflags::Bits>::EMPTY
                }
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let f = self;
                {
                    Self::all().bits() | f.bits() == f.bits()
                }
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                {
                    f.bits() & other.bits() != <u32 as ::bitflags::Bits>::EMPTY
                }
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                {
                    f.bits() & other.bits() == other.bits()
                }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).union(other);
                }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).difference(other);
                }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).symmetric_difference(other);
                }
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let f = self;
                let other = other;
                let value = value;
                {
                    if value {
                        f.insert(other);
                    } else {
                        f.remove(other);
                    }
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self::from_bits_retain(f.bits() & other.bits())
                }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self::from_bits_retain(f.bits() | other.bits())
                }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self::from_bits_retain(f.bits() & !other.bits())
                }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self::from_bits_retain(f.bits() ^ other.bits())
                }
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let f = self;
                {
                    Self::from_bits_truncate(!f.bits())
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: InternalBitFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<InternalBitFlags> for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags> for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                iterator: T,
            ) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl InternalBitFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
                ::bitflags::iter::Iter::__private_const_new(
                    <Permission as ::bitflags::Flags>::FLAGS,
                    Permission::from_bits_retain(self.bits()),
                    Permission::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <Permission as ::bitflags::Flags>::FLAGS,
                    Permission::from_bits_retain(self.bits()),
                    Permission::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
            type Item = Permission;
            type IntoIter = ::bitflags::iter::Iter<Permission>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
        impl InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut u32 {
                &mut self.0
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl Permission {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                {
                    Self(InternalBitFlags::empty())
                }
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                {
                    Self(InternalBitFlags::all())
                }
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                let f = self;
                {
                    f.0.bits()
                }
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(bits: u32) -> ::bitflags::__private::core::option::Option<Self> {
                let bits = bits;
                {
                    match InternalBitFlags::from_bits(bits) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        },
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        },
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                let bits = bits;
                {
                    Self(InternalBitFlags::from_bits_truncate(bits))
                }
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                let bits = bits;
                {
                    Self(InternalBitFlags::from_bits_retain(bits))
                }
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(name: &str) -> ::bitflags::__private::core::option::Option<Self> {
                let name = name;
                {
                    match InternalBitFlags::from_name(name) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        },
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        },
                    }
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let f = self;
                {
                    f.0.is_empty()
                }
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let f = self;
                {
                    f.0.is_all()
                }
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                {
                    f.0.intersects(other.0)
                }
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                {
                    f.0.contains(other.0)
                }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    f.0.insert(other.0)
                }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    f.0.remove(other.0)
                }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    f.0.toggle(other.0)
                }
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let f = self;
                let other = other;
                let value = value;
                {
                    f.0.set(other.0, value)
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self(f.0.intersection(other.0))
                }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self(f.0.union(other.0))
                }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self(f.0.difference(other.0))
                }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                {
                    Self(f.0.symmetric_difference(other.0))
                }
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let f = self;
                {
                    Self(f.0.complement())
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for Permission {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for Permission {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for Permission {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for Permission {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for Permission {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: Permission) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for Permission {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for Permission {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for Permission {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for Permission {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for Permission {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for Permission {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for Permission {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for Permission {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<Permission> for Permission {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<Permission> for Permission {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                iterator: T,
            ) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl Permission {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
                ::bitflags::iter::Iter::__private_const_new(
                    <Permission as ::bitflags::Flags>::FLAGS,
                    Permission::from_bits_retain(self.bits()),
                    Permission::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <Permission as ::bitflags::Flags>::FLAGS,
                    Permission::from_bits_retain(self.bits()),
                    Permission::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for Permission {
            type Item = Permission;
            type IntoIter = ::bitflags::iter::Iter<Permission>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
    /// An empty contract. To be used as a template when starting a new contract from scratch.
    pub trait EmptyContract: multiversx_sc::contract_base::ContractBase + Sized {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn init(&self) {}
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn upgrade(&self) {}
    }
    pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}
    impl<C> EmptyContract for C where C: AutoImpl {}
    impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }
    pub trait EndpointWrappers: multiversx_sc::contract_base::ContractBase + EmptyContract {
        #[inline]
        fn call_init(&mut self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
            self.init();
        }
        #[inline]
        fn call_upgrade(&mut self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
            self.upgrade();
        }
        fn call(&mut self, fn_name: &str) -> bool {
            match fn_name {
                "callBack" => {
                    self::EndpointWrappers::callback(self);
                    true
                },
                "init"
                    if <Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() =>
                {
                    multiversx_sc::external_view_contract::external_view_contract_constructor::<
                        Self::Api,
                    >();
                    true
                },
                "init"
                    if !<Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() =>
                {
                    self.call_init();
                    true
                },
                "upgrade" => {
                    self.call_upgrade();
                    true
                },
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
    impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }
    pub struct AbiProvider {}
    impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = multiversx_sc::api::uncallable::UncallableApi;
        fn abi() -> multiversx_sc::abi::ContractAbi {
            let mut contract_abi = multiversx_sc::abi::ContractAbi::new(
                multiversx_sc::abi::BuildInfoAbi {
                    contract_crate: multiversx_sc::abi::ContractCrateBuildAbi {
                        name: "empty",
                        version: "0.0.0",
                        git_version: "",
                    },
                    framework: multiversx_sc::abi::FrameworkBuildAbi::create(),
                },
                &[
                    "An empty contract. To be used as a template when starting a new contract from scratch.",
                ],
                "EmptyContract",
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
                "upgrade",
                "upgrade",
                multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                multiversx_sc::abi::EndpointTypeAbi::Upgrade,
            );
            contract_abi.upgrade_constructors.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A>(multiversx_sc::contract_base::UniversalContractObj<A>)
    where
        A: multiversx_sc::api::VMApi;
    impl<A> multiversx_sc::contract_base::ContractBase for ContractObj<A>
    where
        A: multiversx_sc::api::VMApi,
    {
        type Api = A;
    }
    impl<A> AutoImpl for ContractObj<A> where A: multiversx_sc::api::VMApi {}
    impl<A> EndpointWrappers for ContractObj<A> where A: multiversx_sc::api::VMApi {}
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
        ) -> multiversx_sc::types::heap::Box<dyn multiversx_sc::contract_base::CallableContract>
        {
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
        pub fn upgrade<A>()
        where
            A: multiversx_sc::api::VMApi,
        {
            super::EndpointWrappers::call_upgrade(
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
        fn upgrade(
            &mut self,
        ) -> multiversx_sc::types::Tx<
            multiversx_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            multiversx_sc::types::FunctionCall<Self::Api>,
            multiversx_sc::types::OriginalResultMarker<()>,
        > {
            multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("upgrade")
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
        pub address:
            multiversx_sc::types::ManagedOption<A, multiversx_sc::types::ManagedAddress<A>>,
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
    impl<A> ProxyTrait for Proxy<A> where A: multiversx_sc::api::VMApi {}
    impl<A> ProxyTrait for ProxyTo<A> where A: multiversx_sc::api::VMApi {}
}
use bitflags::bitflags;
use multiversx_sc::derive::type_abi;
pub struct Permission(<Permission as ::bitflags::__private::PublicFlags>::Internal);
#[automatically_derived]
impl ::core::clone::Clone for Permission {
    #[inline]
    fn clone(&self) -> Permission {
        Permission(::core::clone::Clone::clone(&self.0))
    }
}
impl multiversx_sc::abi::TypeAbiFrom<Self> for Permission {}
impl multiversx_sc::abi::TypeAbiFrom<&Self> for Permission {}
impl multiversx_sc::abi::TypeAbi for Permission {
    type Unmanaged = Self;
    fn type_name() -> multiversx_sc::abi::TypeName {
        "Permission".into()
    }
    fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
        accumulator: &mut TDC,
    ) {
        let type_names = Self::type_names();
        if !accumulator.contains_type(&type_names.abi) {
            accumulator.reserve_type_name(type_names.clone());
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "0",
                <<Permission as ::bitflags::__private::PublicFlags>::Internal>::type_names(),
            ));
            <<Permission as ::bitflags::__private::PublicFlags>::Internal>::provide_type_descriptions(
                accumulator,
            );
            accumulator.insert(
                type_names.clone(),
                multiversx_sc::abi::TypeDescription::new(
                    &[],
                    type_names,
                    multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                    &["Clone"],
                ),
            );
        }
    }
}
impl Permission {
    #[allow(deprecated, non_upper_case_globals)]
    pub const NONE: Self = Self::from_bits_retain(0);
    #[allow(deprecated, non_upper_case_globals)]
    pub const OWNER: Self = Self::from_bits_retain(1);
    #[allow(deprecated, non_upper_case_globals)]
    pub const ADMIN: Self = Self::from_bits_retain(2);
    #[allow(deprecated, non_upper_case_globals)]
    pub const PAUSE: Self = Self::from_bits_retain(4);
}
impl ::bitflags::Flags for Permission {
    const FLAGS: &'static [::bitflags::Flag<Permission>] = &[
        {
            #[allow(deprecated, non_upper_case_globals)]
            ::bitflags::Flag::new("NONE", Permission::NONE)
        },
        {
            #[allow(deprecated, non_upper_case_globals)]
            ::bitflags::Flag::new("OWNER", Permission::OWNER)
        },
        {
            #[allow(deprecated, non_upper_case_globals)]
            ::bitflags::Flag::new("ADMIN", Permission::ADMIN)
        },
        {
            #[allow(deprecated, non_upper_case_globals)]
            ::bitflags::Flag::new("PAUSE", Permission::PAUSE)
        },
    ];
    type Bits = u32;
    fn bits(&self) -> u32 {
        Permission::bits(self)
    }
    fn from_bits_retain(bits: u32) -> Permission {
        Permission::from_bits_retain(bits)
    }
}
#[allow(
    dead_code,
    deprecated,
    unused_doc_comments,
    unused_attributes,
    unused_mut,
    unused_imports,
    non_upper_case_globals,
    clippy::assign_op_pattern,
    clippy::indexing_slicing,
    clippy::same_name_method,
    clippy::iter_without_into_iter
)]
const _: () = {
    #[repr(transparent)]
    pub struct InternalBitFlags(u32);
    #[automatically_derived]
    impl ::core::clone::Clone for InternalBitFlags {
        #[inline]
        fn clone(&self) -> InternalBitFlags {
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for InternalBitFlags {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InternalBitFlags {
        #[inline]
        fn eq(&self, other: &InternalBitFlags) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for InternalBitFlags {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for InternalBitFlags {
        #[inline]
        fn partial_cmp(
            &self,
            other: &InternalBitFlags,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for InternalBitFlags {
        #[inline]
        fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for InternalBitFlags {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl ::bitflags::__private::PublicFlags for Permission {
        type Primitive = u32;
        type Internal = InternalBitFlags;
    }
    impl ::bitflags::__private::core::default::Default for InternalBitFlags {
        #[inline]
        fn default() -> Self {
            InternalBitFlags::empty()
        }
    }
    impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
        ) -> ::bitflags::__private::core::fmt::Result {
            if self.is_empty() {
                f.write_fmt(format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY))
            } else {
                ::bitflags::__private::core::fmt::Display::fmt(self, f)
            }
        }
    }
    impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
        ) -> ::bitflags::__private::core::fmt::Result {
            ::bitflags::parser::to_writer(&Permission(*self), f)
        }
    }
    impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
        type Err = ::bitflags::parser::ParseError;
        fn from_str(s: &str) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
            ::bitflags::parser::from_str::<Permission>(s).map(|flags| flags.0)
        }
    }
    impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
        fn as_ref(&self) -> &u32 {
            &self.0
        }
    }
    impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
        fn from(bits: u32) -> Self {
            Self::from_bits_retain(bits)
        }
    }
    #[allow(dead_code, deprecated, unused_attributes)]
    impl InternalBitFlags {
        /// Get a flags value with all bits unset.
        #[inline]
        pub const fn empty() -> Self {
            {
                Self(<u32 as ::bitflags::Bits>::EMPTY)
            }
        }
        /// Get a flags value with all known bits set.
        #[inline]
        pub const fn all() -> Self {
            {
                let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                let mut i = 0;
                {
                    {
                        let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <Permission as ::bitflags::Flags>::FLAGS[i].value().bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                let _ = i;
                Self::from_bits_retain(truncated)
            }
        }
        /// Get the underlying bits value.
        ///
        /// The returned value is exactly the bits set in this flags value.
        #[inline]
        pub const fn bits(&self) -> u32 {
            let f = self;
            {
                f.0
            }
        }
        /// Convert from a bits value.
        ///
        /// This method will return `None` if any unknown bits are set.
        #[inline]
        pub const fn from_bits(bits: u32) -> ::bitflags::__private::core::option::Option<Self> {
            let bits = bits;
            {
                let truncated = Self::from_bits_truncate(bits).0;
                if truncated == bits {
                    ::bitflags::__private::core::option::Option::Some(Self(bits))
                } else {
                    ::bitflags::__private::core::option::Option::None
                }
            }
        }
        /// Convert from a bits value, unsetting any unknown bits.
        #[inline]
        pub const fn from_bits_truncate(bits: u32) -> Self {
            let bits = bits;
            {
                Self(bits & Self::all().bits())
            }
        }
        /// Convert from a bits value exactly.
        #[inline]
        pub const fn from_bits_retain(bits: u32) -> Self {
            let bits = bits;
            {
                Self(bits)
            }
        }
        /// Get a flags value with the bits of a flag with the given name set.
        ///
        /// This method will return `None` if `name` is empty or doesn't
        /// correspond to any named flag.
        #[inline]
        pub fn from_name(name: &str) -> ::bitflags::__private::core::option::Option<Self> {
            let name = name;
            {
                {
                    if name == "NONE" {
                        return ::bitflags::__private::core::option::Option::Some(Self(
                            Permission::NONE.bits(),
                        ));
                    }
                };
                {
                    if name == "OWNER" {
                        return ::bitflags::__private::core::option::Option::Some(Self(
                            Permission::OWNER.bits(),
                        ));
                    }
                };
                {
                    if name == "ADMIN" {
                        return ::bitflags::__private::core::option::Option::Some(Self(
                            Permission::ADMIN.bits(),
                        ));
                    }
                };
                {
                    if name == "PAUSE" {
                        return ::bitflags::__private::core::option::Option::Some(Self(
                            Permission::PAUSE.bits(),
                        ));
                    }
                };
                let _ = name;
                ::bitflags::__private::core::option::Option::None
            }
        }
        /// Whether all bits in this flags value are unset.
        #[inline]
        pub const fn is_empty(&self) -> bool {
            let f = self;
            {
                f.bits() == <u32 as ::bitflags::Bits>::EMPTY
            }
        }
        /// Whether all known bits in this flags value are set.
        #[inline]
        pub const fn is_all(&self) -> bool {
            let f = self;
            {
                Self::all().bits() | f.bits() == f.bits()
            }
        }
        /// Whether any set bits in a source flags value are also set in a target flags value.
        #[inline]
        pub const fn intersects(&self, other: Self) -> bool {
            let f = self;
            let other = other;
            {
                f.bits() & other.bits() != <u32 as ::bitflags::Bits>::EMPTY
            }
        }
        /// Whether all set bits in a source flags value are also set in a target flags value.
        #[inline]
        pub const fn contains(&self, other: Self) -> bool {
            let f = self;
            let other = other;
            {
                f.bits() & other.bits() == other.bits()
            }
        }
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        pub fn insert(&mut self, other: Self) {
            let f = self;
            let other = other;
            {
                *f = Self::from_bits_retain(f.bits()).union(other);
            }
        }
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `remove` won't truncate `other`, but the `!` operator will.
        #[inline]
        pub fn remove(&mut self, other: Self) {
            let f = self;
            let other = other;
            {
                *f = Self::from_bits_retain(f.bits()).difference(other);
            }
        }
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        pub fn toggle(&mut self, other: Self) {
            let f = self;
            let other = other;
            {
                *f = Self::from_bits_retain(f.bits()).symmetric_difference(other);
            }
        }
        /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
        #[inline]
        pub fn set(&mut self, other: Self, value: bool) {
            let f = self;
            let other = other;
            let value = value;
            {
                if value {
                    f.insert(other);
                } else {
                    f.remove(other);
                }
            }
        }
        /// The bitwise and (`&`) of the bits in two flags values.
        #[inline]
        #[must_use]
        pub const fn intersection(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self::from_bits_retain(f.bits() & other.bits())
            }
        }
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        #[must_use]
        pub const fn union(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self::from_bits_retain(f.bits() | other.bits())
            }
        }
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `difference` won't truncate `other`, but the `!` operator will.
        #[inline]
        #[must_use]
        pub const fn difference(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self::from_bits_retain(f.bits() & !other.bits())
            }
        }
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        #[must_use]
        pub const fn symmetric_difference(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self::from_bits_retain(f.bits() ^ other.bits())
            }
        }
        /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
        #[inline]
        #[must_use]
        pub const fn complement(self) -> Self {
            let f = self;
            {
                Self::from_bits_truncate(!f.bits())
            }
        }
    }
    impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
        type Output = Self;
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        fn bitor(self, other: InternalBitFlags) -> Self {
            self.union(other)
        }
    }
    impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        fn bitor_assign(&mut self, other: Self) {
            self.insert(other);
        }
    }
    impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
        type Output = Self;
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        fn bitxor(self, other: Self) -> Self {
            self.symmetric_difference(other)
        }
    }
    impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        fn bitxor_assign(&mut self, other: Self) {
            self.toggle(other);
        }
    }
    impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
        type Output = Self;
        /// The bitwise and (`&`) of the bits in two flags values.
        #[inline]
        fn bitand(self, other: Self) -> Self {
            self.intersection(other)
        }
    }
    impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
        /// The bitwise and (`&`) of the bits in two flags values.
        #[inline]
        fn bitand_assign(&mut self, other: Self) {
            *self = Self::from_bits_retain(self.bits()).intersection(other);
        }
    }
    impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
        type Output = Self;
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `difference` won't truncate `other`, but the `!` operator will.
        #[inline]
        fn sub(self, other: Self) -> Self {
            self.difference(other)
        }
    }
    impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `difference` won't truncate `other`, but the `!` operator will.
        #[inline]
        fn sub_assign(&mut self, other: Self) {
            self.remove(other);
        }
    }
    impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
        type Output = Self;
        /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
        #[inline]
        fn not(self) -> Self {
            self.complement()
        }
    }
    impl ::bitflags::__private::core::iter::Extend<InternalBitFlags> for InternalBitFlags {
        /// The bitwise or (`|`) of the bits in each flags value.
        fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
            &mut self,
            iterator: T,
        ) {
            for item in iterator {
                self.insert(item)
            }
        }
    }
    impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags> for InternalBitFlags {
        /// The bitwise or (`|`) of the bits in each flags value.
        fn from_iter<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
            iterator: T,
        ) -> Self {
            use ::bitflags::__private::core::iter::Extend;
            let mut result = Self::empty();
            result.extend(iterator);
            result
        }
    }
    impl InternalBitFlags {
        /// Yield a set of contained flags values.
        ///
        /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
        /// will be yielded together as a final flags value.
        #[inline]
        pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
            ::bitflags::iter::Iter::__private_const_new(
                <Permission as ::bitflags::Flags>::FLAGS,
                Permission::from_bits_retain(self.bits()),
                Permission::from_bits_retain(self.bits()),
            )
        }
        /// Yield a set of contained named flags values.
        ///
        /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
        /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
        #[inline]
        pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
            ::bitflags::iter::IterNames::__private_const_new(
                <Permission as ::bitflags::Flags>::FLAGS,
                Permission::from_bits_retain(self.bits()),
                Permission::from_bits_retain(self.bits()),
            )
        }
    }
    impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
        type Item = Permission;
        type IntoIter = ::bitflags::iter::Iter<Permission>;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    impl InternalBitFlags {
        /// Returns a mutable reference to the raw value of the flags currently stored.
        #[inline]
        pub fn bits_mut(&mut self) -> &mut u32 {
            &mut self.0
        }
    }
    #[allow(dead_code, deprecated, unused_attributes)]
    impl Permission {
        /// Get a flags value with all bits unset.
        #[inline]
        pub const fn empty() -> Self {
            {
                Self(InternalBitFlags::empty())
            }
        }
        /// Get a flags value with all known bits set.
        #[inline]
        pub const fn all() -> Self {
            {
                Self(InternalBitFlags::all())
            }
        }
        /// Get the underlying bits value.
        ///
        /// The returned value is exactly the bits set in this flags value.
        #[inline]
        pub const fn bits(&self) -> u32 {
            let f = self;
            {
                f.0.bits()
            }
        }
        /// Convert from a bits value.
        ///
        /// This method will return `None` if any unknown bits are set.
        #[inline]
        pub const fn from_bits(bits: u32) -> ::bitflags::__private::core::option::Option<Self> {
            let bits = bits;
            {
                match InternalBitFlags::from_bits(bits) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    },
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    },
                }
            }
        }
        /// Convert from a bits value, unsetting any unknown bits.
        #[inline]
        pub const fn from_bits_truncate(bits: u32) -> Self {
            let bits = bits;
            {
                Self(InternalBitFlags::from_bits_truncate(bits))
            }
        }
        /// Convert from a bits value exactly.
        #[inline]
        pub const fn from_bits_retain(bits: u32) -> Self {
            let bits = bits;
            {
                Self(InternalBitFlags::from_bits_retain(bits))
            }
        }
        /// Get a flags value with the bits of a flag with the given name set.
        ///
        /// This method will return `None` if `name` is empty or doesn't
        /// correspond to any named flag.
        #[inline]
        pub fn from_name(name: &str) -> ::bitflags::__private::core::option::Option<Self> {
            let name = name;
            {
                match InternalBitFlags::from_name(name) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    },
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    },
                }
            }
        }
        /// Whether all bits in this flags value are unset.
        #[inline]
        pub const fn is_empty(&self) -> bool {
            let f = self;
            {
                f.0.is_empty()
            }
        }
        /// Whether all known bits in this flags value are set.
        #[inline]
        pub const fn is_all(&self) -> bool {
            let f = self;
            {
                f.0.is_all()
            }
        }
        /// Whether any set bits in a source flags value are also set in a target flags value.
        #[inline]
        pub const fn intersects(&self, other: Self) -> bool {
            let f = self;
            let other = other;
            {
                f.0.intersects(other.0)
            }
        }
        /// Whether all set bits in a source flags value are also set in a target flags value.
        #[inline]
        pub const fn contains(&self, other: Self) -> bool {
            let f = self;
            let other = other;
            {
                f.0.contains(other.0)
            }
        }
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        pub fn insert(&mut self, other: Self) {
            let f = self;
            let other = other;
            {
                f.0.insert(other.0)
            }
        }
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `remove` won't truncate `other`, but the `!` operator will.
        #[inline]
        pub fn remove(&mut self, other: Self) {
            let f = self;
            let other = other;
            {
                f.0.remove(other.0)
            }
        }
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        pub fn toggle(&mut self, other: Self) {
            let f = self;
            let other = other;
            {
                f.0.toggle(other.0)
            }
        }
        /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
        #[inline]
        pub fn set(&mut self, other: Self, value: bool) {
            let f = self;
            let other = other;
            let value = value;
            {
                f.0.set(other.0, value)
            }
        }
        /// The bitwise and (`&`) of the bits in two flags values.
        #[inline]
        #[must_use]
        pub const fn intersection(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self(f.0.intersection(other.0))
            }
        }
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        #[must_use]
        pub const fn union(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self(f.0.union(other.0))
            }
        }
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `difference` won't truncate `other`, but the `!` operator will.
        #[inline]
        #[must_use]
        pub const fn difference(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self(f.0.difference(other.0))
            }
        }
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        #[must_use]
        pub const fn symmetric_difference(self, other: Self) -> Self {
            let f = self;
            let other = other;
            {
                Self(f.0.symmetric_difference(other.0))
            }
        }
        /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
        #[inline]
        #[must_use]
        pub const fn complement(self) -> Self {
            let f = self;
            {
                Self(f.0.complement())
            }
        }
    }
    impl ::bitflags::__private::core::fmt::Binary for Permission {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::fmt::Octal for Permission {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::fmt::LowerHex for Permission {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::fmt::UpperHex for Permission {
        fn fmt(
            &self,
            f: &mut ::bitflags::__private::core::fmt::Formatter,
        ) -> ::bitflags::__private::core::fmt::Result {
            let inner = self.0;
            ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
        }
    }
    impl ::bitflags::__private::core::ops::BitOr for Permission {
        type Output = Self;
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        fn bitor(self, other: Permission) -> Self {
            self.union(other)
        }
    }
    impl ::bitflags::__private::core::ops::BitOrAssign for Permission {
        /// The bitwise or (`|`) of the bits in two flags values.
        #[inline]
        fn bitor_assign(&mut self, other: Self) {
            self.insert(other);
        }
    }
    impl ::bitflags::__private::core::ops::BitXor for Permission {
        type Output = Self;
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        fn bitxor(self, other: Self) -> Self {
            self.symmetric_difference(other)
        }
    }
    impl ::bitflags::__private::core::ops::BitXorAssign for Permission {
        /// The bitwise exclusive-or (`^`) of the bits in two flags values.
        #[inline]
        fn bitxor_assign(&mut self, other: Self) {
            self.toggle(other);
        }
    }
    impl ::bitflags::__private::core::ops::BitAnd for Permission {
        type Output = Self;
        /// The bitwise and (`&`) of the bits in two flags values.
        #[inline]
        fn bitand(self, other: Self) -> Self {
            self.intersection(other)
        }
    }
    impl ::bitflags::__private::core::ops::BitAndAssign for Permission {
        /// The bitwise and (`&`) of the bits in two flags values.
        #[inline]
        fn bitand_assign(&mut self, other: Self) {
            *self = Self::from_bits_retain(self.bits()).intersection(other);
        }
    }
    impl ::bitflags::__private::core::ops::Sub for Permission {
        type Output = Self;
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `difference` won't truncate `other`, but the `!` operator will.
        #[inline]
        fn sub(self, other: Self) -> Self {
            self.difference(other)
        }
    }
    impl ::bitflags::__private::core::ops::SubAssign for Permission {
        /// The intersection of a source flags value with the complement of a target flags value (`&!`).
        ///
        /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
        /// `difference` won't truncate `other`, but the `!` operator will.
        #[inline]
        fn sub_assign(&mut self, other: Self) {
            self.remove(other);
        }
    }
    impl ::bitflags::__private::core::ops::Not for Permission {
        type Output = Self;
        /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
        #[inline]
        fn not(self) -> Self {
            self.complement()
        }
    }
    impl ::bitflags::__private::core::iter::Extend<Permission> for Permission {
        /// The bitwise or (`|`) of the bits in each flags value.
        fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
            &mut self,
            iterator: T,
        ) {
            for item in iterator {
                self.insert(item)
            }
        }
    }
    impl ::bitflags::__private::core::iter::FromIterator<Permission> for Permission {
        /// The bitwise or (`|`) of the bits in each flags value.
        fn from_iter<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
            iterator: T,
        ) -> Self {
            use ::bitflags::__private::core::iter::Extend;
            let mut result = Self::empty();
            result.extend(iterator);
            result
        }
    }
    impl Permission {
        /// Yield a set of contained flags values.
        ///
        /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
        /// will be yielded together as a final flags value.
        #[inline]
        pub const fn iter(&self) -> ::bitflags::iter::Iter<Permission> {
            ::bitflags::iter::Iter::__private_const_new(
                <Permission as ::bitflags::Flags>::FLAGS,
                Permission::from_bits_retain(self.bits()),
                Permission::from_bits_retain(self.bits()),
            )
        }
        /// Yield a set of contained named flags values.
        ///
        /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
        /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
        #[inline]
        pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<Permission> {
            ::bitflags::iter::IterNames::__private_const_new(
                <Permission as ::bitflags::Flags>::FLAGS,
                Permission::from_bits_retain(self.bits()),
                Permission::from_bits_retain(self.bits()),
            )
        }
    }
    impl ::bitflags::__private::core::iter::IntoIterator for Permission {
        type Item = Permission;
        type IntoIter = ::bitflags::iter::Iter<Permission>;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
};
/// An empty contract. To be used as a template when starting a new contract from scratch.
pub trait EmptyContract: multiversx_sc::contract_base::ContractBase + Sized {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(&self) {}
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn upgrade(&self) {}
}
pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}
impl<C> EmptyContract for C where C: AutoImpl {}
impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
    A: multiversx_sc::api::VMApi
{
}
pub trait EndpointWrappers: multiversx_sc::contract_base::ContractBase + EmptyContract {
    #[inline]
    fn call_init(&mut self) {
        <Self::Api as multiversx_sc::api::VMApi>::init_static();
        multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
        let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
        self.init();
    }
    #[inline]
    fn call_upgrade(&mut self) {
        <Self::Api as multiversx_sc::api::VMApi>::init_static();
        multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
        let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
        self.upgrade();
    }
    fn call(&mut self, fn_name: &str) -> bool {
        match fn_name {
            "callBack" => {
                self::EndpointWrappers::callback(self);
                true
            },
            "init" if <Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() => {
                multiversx_sc::external_view_contract::external_view_contract_constructor::<
                    Self::Api,
                >();
                true
            },
            "init" if !<Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() => {
                self.call_init();
                true
            },
            "upgrade" => {
                self.call_upgrade();
                true
            },
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
impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
    A: multiversx_sc::api::VMApi
{
}
pub struct AbiProvider {}
impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
    type Api = multiversx_sc::api::uncallable::UncallableApi;
    fn abi() -> multiversx_sc::abi::ContractAbi {
        let mut contract_abi = multiversx_sc::abi::ContractAbi::new(
            multiversx_sc::abi::BuildInfoAbi {
                contract_crate: multiversx_sc::abi::ContractCrateBuildAbi {
                    name: "empty",
                    version: "0.0.0",
                    git_version: "",
                },
                framework: multiversx_sc::abi::FrameworkBuildAbi::create(),
            },
            &[
                "An empty contract. To be used as a template when starting a new contract from scratch.",
            ],
            "EmptyContract",
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
            "upgrade",
            "upgrade",
            multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
            multiversx_sc::abi::EndpointTypeAbi::Upgrade,
        );
        contract_abi.upgrade_constructors.push(endpoint_abi);
        contract_abi
    }
}
pub struct ContractObj<A>(multiversx_sc::contract_base::UniversalContractObj<A>)
where
    A: multiversx_sc::api::VMApi;
impl<A> multiversx_sc::contract_base::ContractBase for ContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{
    type Api = A;
}
impl<A> AutoImpl for ContractObj<A> where A: multiversx_sc::api::VMApi {}
impl<A> EndpointWrappers for ContractObj<A> where A: multiversx_sc::api::VMApi {}
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
    ) -> multiversx_sc::types::heap::Box<dyn multiversx_sc::contract_base::CallableContract> {
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
    pub fn upgrade<A>()
    where
        A: multiversx_sc::api::VMApi,
    {
        super::EndpointWrappers::call_upgrade(
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
    fn upgrade(
        &mut self,
    ) -> multiversx_sc::types::Tx<
        multiversx_sc::types::TxScEnv<Self::Api>,
        (),
        Self::To,
        (),
        (),
        multiversx_sc::types::FunctionCall<Self::Api>,
        multiversx_sc::types::OriginalResultMarker<()>,
    > {
        multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
            .to(self.extract_proxy_to())
            .original_result()
            .raw_call("upgrade")
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
    pub address: multiversx_sc::types::ManagedOption<A, multiversx_sc::types::ManagedAddress<A>>,
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
impl<A> ProxyTrait for Proxy<A> where A: multiversx_sc::api::VMApi {}
impl<A> ProxyTrait for ProxyTo<A> where A: multiversx_sc::api::VMApi {}
