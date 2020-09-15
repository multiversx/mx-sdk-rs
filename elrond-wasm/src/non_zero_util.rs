use core::num::NonZeroUsize;
use core::iter::Iterator;

pub struct NonZeroUsizeIterator {
    prev_num: usize,
    limit: usize,
}

impl NonZeroUsizeIterator {
    pub fn from_1_to_n(n: usize) -> Self {
        NonZeroUsizeIterator{
            prev_num: 0,
            limit: n,
        }
    }
}

impl Iterator for NonZeroUsizeIterator {
    type Item = NonZeroUsize;
    
    fn next(&mut self) -> Option<NonZeroUsize> {
        if self.prev_num >= self.limit {
            return None;
        }

        self.prev_num += 1;
        unsafe {
            Some(NonZeroUsize::new_unchecked(self.prev_num))
        }
        
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_iter_0() {
        let mut v = Vec::<usize>::new();
        for nz in NonZeroUsizeIterator::from_1_to_n(0) {
            v.push(nz.get());
        }

        assert_eq!(v, [].to_vec());
    }

    #[test]
    fn test_iter_1() {
        let mut v = Vec::<usize>::new();
        for nz in NonZeroUsizeIterator::from_1_to_n(1) {
            v.push(nz.get());
        }

        assert_eq!(v, [1].to_vec());
    }

    #[test]
    fn test_iter_3() {
        let mut v = Vec::<usize>::new();
        for nz in NonZeroUsizeIterator::from_1_to_n(3) {
            v.push(nz.get());
        }

        assert_eq!(v, [1, 2, 3].to_vec());
    }
}
