use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// Wraps an object and provides mutable access to it.
///
/// The point is that sometimes we want to stop mutability and proliferate reference-counted pointers to it.
///
/// This happens in a controlled environment, in the `with_shared` method closure argument.
/// All reference-counted pointers are expected to be dropped until that closure finishes.
pub enum Shareable<T> {
    Owned(T),
    Shared(Arc<T>),
}

impl<T> Shareable<T> {
    pub fn new(t: T) -> Self {
        Shareable::Owned(t)
    }

    /// Destroys the object and returns the contents.
    pub fn into_inner(self) -> T {
        if let Shareable::Owned(t) = self {
            t
        } else {
            panic!("cannot access ShareableMut owned object")
        }
    }
}

impl<T> Default for Shareable<T>
where
    T: Default,
{
    fn default() -> Self {
        Shareable::new(T::default())
    }
}

impl<T> Deref for Shareable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Shareable::Owned(t) => t,
            Shareable::Shared(rc) => rc.deref(),
        }
    }
}

impl<T> DerefMut for Shareable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Shareable::Owned(t) => t,
            Shareable::Shared(_) => {
                panic!("cannot mutably dereference ShareableMut when in Shared state")
            },
        }
    }
}

impl<T> Shareable<T> {
    fn get_arc(&self) -> Arc<T> {
        if let Shareable::Shared(arc) = self {
            arc.clone()
        } else {
            panic!("invalid ShareableMut state: Shared expected")
        }
    }

    fn wrap_arc_strict(&mut self) {
        unsafe {
            let temp = std::ptr::read(self);
            if let Shareable::Owned(t) = temp {
                std::ptr::write(self, Shareable::Shared(Arc::new(t)));
            } else {
                std::mem::forget(temp);
                panic!("invalid ShareableMut state: Owned expected")
            }
        }
    }

    fn unwrap_arc_strict(&mut self) {
        unsafe {
            let temp = std::ptr::read(self);
            if let Shareable::Shared(arc) = temp {
                match Arc::try_unwrap(arc) {
                    Ok(t) => {
                        std::ptr::write(self, Shareable::Owned(t));
                    },
                    Err(rc) => {
                        std::mem::forget(rc);
                        panic!("failed to recover Owned ShareableMut from Shared, not all Rc pointers dropped")
                    },
                }
            } else {
                std::mem::forget(temp);
                panic!("invalid ShareableMut state: Shared expected")
            }
        }
    }

    /// The main functionality of `Shared`.
    ///
    /// Temporarily makes the object immutable, and creates a Rc pointer to the contents, which can then be cloned.
    ///
    /// Important restriction: all Rc pointers creates from the one given to the closure `f` must be dropped before its execution ends.
    /// Otherwise the operation will panic.
    pub fn with_shared<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(Arc<T>) -> R,
    {
        self.wrap_arc_strict();

        let result = f(self.get_arc());

        self.unwrap_arc_strict();

        result
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::Shareable;

    #[test]
    fn test_shareable_mut_1() {
        let mut s = Shareable::new("test string".to_string());
        let l = s.with_shared(|s_arc| s_arc.len());
        assert_eq!(s.len(), l);
    }

    #[test]
    fn test_shareable_mut_2() {
        let mut s = Shareable::new(RefCell::new("test string".to_string()));
        s.with_shared(|s_arc| {
            s_arc.borrow_mut().push_str(" ... changed");
        });
        assert_eq!(s.borrow().as_str(), "test string ... changed");
        assert_eq!(s.into_inner().into_inner(), "test string ... changed");
    }

    #[test]
    #[should_panic = "failed to recover Owned ShareableMut from Shared, not all Rc pointers dropped"]
    fn test_shareable_mut_fail() {
        let mut s = Shareable::new("test string".to_string());
        let _illegally_extracted_arc = s.with_shared(|s_arc| s_arc);
    }
}
