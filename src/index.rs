#![allow(clippy::redundant_pub_crate)] // NOTE: ensure this is never exposed due to no-panic safety

use core::ops::{RangeFrom, RangeTo};

pub(crate) trait GetAt<Index, R: ?Sized> {
    fn at(&self, index: Index) -> &R;
}

impl<T> GetAt<usize, T> for [T] {
    fn at(&self, index: usize) -> &T {
        #[cfg(not(feature = "no-panic"))]
        let r = &self[index];
        #[cfg(feature = "no-panic")]
        let r = unsafe { self.get_unchecked(index) };
        r
    }
}

impl<T> GetAt<RangeFrom<usize>, [T]> for [T] {
    fn at(&self, index: RangeFrom<usize>) -> &[T] {
        #[cfg(not(feature = "no-panic"))]
        let r = &self[index];
        #[cfg(feature = "no-panic")]
        let r = unsafe { self.get_unchecked(index) };
        r
    }
}

impl<T> GetAt<RangeTo<usize>, [T]> for [T] {
    fn at(&self, index: RangeTo<usize>) -> &[T] {
        #[cfg(not(feature = "no-panic"))]
        let r = &self[index];
        #[cfg(feature = "no-panic")]
        let r = unsafe { self.get_unchecked(index) };
        r
    }
}

pub(crate) trait GetAtMut<Index, R: ?Sized> {
    fn at_mut(&mut self, index: Index) -> &mut R;
}

impl<T> GetAtMut<usize, T> for [T] {
    fn at_mut(&mut self, index: usize) -> &mut T {
        #[cfg(not(feature = "no-panic"))]
        let r = &mut self[index];
        #[cfg(feature = "no-panic")]
        let r = unsafe { self.get_unchecked_mut(index) };
        r
    }
}

impl<T> GetAtMut<RangeFrom<usize>, [T]> for [T] {
    fn at_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        #[cfg(not(feature = "no-panic"))]
        let r = &mut self[index];
        #[cfg(feature = "no-panic")]
        let r = unsafe { self.get_unchecked_mut(index) };
        r
    }
}
