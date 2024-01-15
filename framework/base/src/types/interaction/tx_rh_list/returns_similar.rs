use core::marker::PhantomData;

pub struct ReturnSimilar<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for ReturnSimilar<T> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T> ReturnSimilar<T> {
    pub fn new() -> Self {
        Self::default()
    }
}
