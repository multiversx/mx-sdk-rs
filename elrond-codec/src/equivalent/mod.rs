mod equivalent_arg;
mod equivalent_result;

pub use equivalent_arg::*;
pub use equivalent_result::*;

#[allow(unused)]
#[cfg(test)]
mod test {
    use crate::*;

    fn take2<T1, T2, R>(x: T1, y: T2) -> R
    where
        T1: EquivalentArgument<i32>,
        T2: EquivalentArgument<usize>,
        R: EquivalentResult<i32>,
    {
        panic!()
    }

    #[test]
    #[should_panic]
    fn test_take() {
        let r: i64 = take2(&5, 6u8);
    }
}
