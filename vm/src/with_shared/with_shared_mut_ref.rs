use std::rc::Rc;

/// Temporarily converts a mutable reference into a reference-counted smart pointer (`Rc`).
///
/// This only takes as long as the closure `f` is executed.
///
/// All subsequent Rc clones must be dropped before `f` terminates, otherwise the function will panic.
///
/// The `Clone` of the argument is not used, except to preserve memory consistency in case of failure.
///
/// See the `Shared` type for a safer implementation, which does not require `Clone`.
pub fn with_shared_mut_ref<T, F, R>(t: &mut T, f: F) -> R
where
    T: Clone,
    F: FnOnce(Rc<T>) -> R,
{
    unsafe {
        // forcefully extract the owned object from the mut ref (unsafe)
        let obj = std::ptr::read(t);

        // wrap the owned object
        let obj_rc = Rc::new(obj);

        // the main action
        let result = f(obj_rc.clone());

        // unwrapping the owned object
        match Rc::try_unwrap(obj_rc) {
            Ok(obj) => {
                // Rc unwrapped successfully
                // no need to write the owned object back to the location given by the pointer t,
                // because it could not have changed in the mean time, it is already there

                // though readonly, the object might have changed via cells,
                // so it needs to be copied back
                std::ptr::write(t, obj);
            },
            Err(obj_rc) => {
                // could not unwrap, this means there are still references to obj elsewhere
                // to avoid memory corruption, we perform a clone of the contents
                let obj = (*obj_rc).clone();
                std::ptr::write(t, obj);
                panic!("failed to recover owned object from Rc")
            },
        }

        result
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::with_shared_mut_ref;

    #[test]
    fn test_with_shared_mut_ref_1() {
        let mut s = "test string".to_string();
        let l = with_shared_mut_ref(&mut s, |s_rc| s_rc.len());
        assert_eq!(s.len(), l);
    }

    #[test]
    fn test_with_shared_mut_ref_2() {
        let mut s = RefCell::new("test string".to_string());
        with_shared_mut_ref(&mut s, |s_rc| {
            s_rc.borrow_mut().push_str(" ... changed");
        });
        assert_eq!(s.borrow().as_str(), "test string ... changed");
    }

    #[test]
    #[should_panic = "failed to recover owned object from Rc"]
    fn test_with_shared_mut_ref_fail() {
        let mut s = "test string".to_string();
        let _illegally_extracted_rc = with_shared_mut_ref(&mut s, |s_rc| s_rc);
    }
}
