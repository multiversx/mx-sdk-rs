multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub trait NativeConvertible {
    type Native;

    fn to_native(&self) -> Self::Native;
}

#[derive(TopEncode, NestedEncode, TopDecode, NestedDecode, Clone, PartialEq, Debug)]
pub struct NativeValue<T: TopEncode + NestedEncode + TopDecode + NestedDecode + Clone> {
    value: T
}

impl<T: TopEncode + NestedEncode + TopDecode + NestedDecode + Clone> NativeValue<T> {
    pub fn new(value: T) -> NativeValue<T> {
        NativeValue {
            value
        }
    }
}

impl<T> NativeConvertible for NativeValue<T>
where
    T: TopEncode + NestedEncode + TopDecode + NestedDecode + Clone
{
    type Native = T;

    fn to_native(&self) -> Self::Native {
       self.value.clone()
    }
}

#[derive(TopEncode, NestedEncode, TopDecode, NestedDecode, Clone, PartialEq, Debug)]
pub struct NativeValueManagedVecItem<T>
where
    T: TopEncode + NestedEncode + TopDecode + NestedDecode + ManagedVecItem + Clone
{
    value: T
}

impl<T> NativeValueManagedVecItem<T>
where
    T: TopEncode + NestedEncode + TopDecode + NestedDecode + ManagedVecItem + Clone
{
    pub fn new(value: T) -> NativeValueManagedVecItem<T> {
        NativeValueManagedVecItem {
            value
        }
    }
}

impl<T> ManagedVecItem for NativeValueManagedVecItem<T>
where
    T: TopEncode + NestedEncode + TopDecode + NestedDecode + ManagedVecItem + Clone
{
    const PAYLOAD_SIZE: usize = T::PAYLOAD_SIZE;
    const SKIPS_RESERIALIZATION: bool = T::SKIPS_RESERIALIZATION;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        NativeValueManagedVecItem::new(T::from_byte_reader(reader))
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(reader: Reader) -> Self::Ref<'a> {
       Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        self.value.to_byte_writer(writer)
    }
}

impl<T> NativeConvertible for NativeValueManagedVecItem<T>
where
    T: TopEncode + NestedEncode + TopDecode + NestedDecode + ManagedVecItem + Clone
{
    type Native = T;

    fn to_native(&self) -> Self::Native {
        self.value.clone()
    }
}