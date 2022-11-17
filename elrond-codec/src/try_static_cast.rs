use core::any::TypeId;

/// Use to transfer objects from one generic type to another,
/// without the compiler being able to determine whether or not the two types are the same.
/// The cast is statically dispatched.
pub trait TryStaticCast: Sized + 'static {
    fn type_eq<U: TryStaticCast>() -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }

    #[inline]
    fn try_cast<U: TryStaticCast>(self) -> Option<U> {
        if Self::type_eq::<U>() {
            let trans: U = unsafe { core::mem::transmute_copy(&self) };
            core::mem::forget(self);
            Some(trans)
        } else {
            None
        }
    }

    #[inline]
    fn try_cast_ref<U: TryStaticCast>(&self) -> Option<&U> {
        if Self::type_eq::<U>() {
            let trans = unsafe { core::mem::transmute::<&Self, &U>(self) };
            Some(trans)
        } else {
            None
        }
    }
}

impl TryStaticCast for () {}
impl TryStaticCast for i32 {}

fn type_eq<T, U>() -> bool
where
    T: 'static,
    U: 'static,
{
    TypeId::of::<T>() == TypeId::of::<U>()
}

#[inline]
pub fn try_cast_ref<T, U>(t: &T) -> Option<&U>
where
    T: 'static,
    U: 'static,
{
    if type_eq::<T, U>() {
        let trans = unsafe { core::mem::transmute::<&T, &U>(t) };
        Some(trans)
    } else {
        None
    }
}

#[inline]
pub fn try_execute_then_cast<T, R, F>(f: F) -> Option<R>
where
    T: 'static,
    R: 'static,
    F: FnOnce() -> T,
{
    if type_eq::<T, R>() {
        let result: T = f();
        let transmuted_result: R = unsafe { core::mem::transmute_copy(&result) };
        core::mem::forget(result);
        Some(transmuted_result)
    } else {
        None
    }
}

#[inline]
pub fn try_cast_execute_or_else<T, U, R, If, Else>(t: T, exec_if: If, exec_else: Else) -> R
where
    T: 'static,
    U: 'static,
    R: 'static,
    If: FnOnce(U) -> R,
    Else: FnOnce(T) -> R,
{
    if type_eq::<T, U>() {
        let transmuted: U = unsafe { core::mem::transmute_copy(&t) };
        core::mem::forget(t);
        exec_if(transmuted)
    } else {
        exec_else(t)
    }
}

#[cfg(test)]
mod test {
    use super::TryStaticCast;

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct SimpleType1(i32);

    impl TryStaticCast for SimpleType1 {}

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct SimpleType2(i32);

    impl TryStaticCast for SimpleType2 {}

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct GenericType<T> {
        id: i32,
        payload: T,
    }

    impl<T> GenericType<T> {
        fn new(id: i32, payload: T) -> Self {
            GenericType { id, payload }
        }
    }

    impl<T: Clone + 'static> TryStaticCast for GenericType<T> {}

    #[test]
    fn test_try_static_cast_simple() {
        let obj = SimpleType1(5);
        assert_eq!(obj.clone().try_cast::<SimpleType1>(), Some(obj.clone()));
        assert_eq!(obj.clone().try_cast::<SimpleType2>(), None);

        assert_eq!(obj.try_cast_ref::<SimpleType1>(), Some(&obj));
        assert_eq!(obj.try_cast_ref::<SimpleType2>(), None);
    }

    #[test]
    fn test_try_static_cast_with_generics() {
        let obj = GenericType::new(100, SimpleType1(5));
        assert_eq!(
            obj.clone().try_cast::<GenericType<SimpleType1>>(),
            Some(obj.clone())
        );
        assert_eq!(obj.try_cast::<GenericType<SimpleType2>>(), None);
    }
}
