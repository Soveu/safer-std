use core::ops::{Bound, RangeBounds};
use core::ptr;

pub struct VecDrain<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> VecDrain<'a, T> {
    pub fn new<R: RangeBounds<usize>>(vec: &'a mut Vec<T>, range: R) -> Self {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n+1,
        };
        let end = match range.start_bound() {
            Bound::Unbounded => vec.len(),
            Bound::Included(&n) => n+1,
            Bound::Excluded(&n) => n,
        };

        assert!(start <= vec.len(), "VecDrain::new> start > vec.len()");
        assert!(end <= vec.len(), "VecDrain::new> end > vec.len()");
        let len = end.checked_sub(start)
            .expect("VecDrain::new> end should be greater than start");

        let to_rotate = &mut vec[start..];
        to_rotate.rotate_left(len);

        let oldlen = vec.len();

        /* SAFETY (set_len): we want to shorten the vec, because later in iteration
         * we will be ptr::read()ing it, effectively duplicating elements without
         * clone(). This can cause double-free when panicing.
         * We have checked that len <= oldlen.
         * SAFETY (get_unchecked): we just grab the slice that we shortened
         */
        let slice = unsafe { 
            vec.set_len(oldlen - len); 
            vec.get_unchecked(vec.len() .. oldlen)
        };

        Self { slice }
    }
}

impl<'a, T: 'a> Drop for VecDrain<'a, T> {
    fn drop(&mut self) {
        for itemref in self.slice.iter() {
            let ptr = itemref as *const T as *mut T;
            /* SAFETY: Vec won't drop again this element (see VecDrain::new) */
            unsafe { ptr::drop_in_place(ptr) }
        }
    }
}

impl<'a, T: 'a> Iterator for VecDrain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.slice.split_first()?;
        self.slice = rest;
        /* SAFETY: Vec won't drop the element and we only pass it once */
        let item = unsafe { ptr::read(first as *const T) };
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.slice.len();
        (n, Some(n))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let n = core::cmp::min(n, self.slice.len());
        let (to_drop, rest) = self.slice.split_at(n);
        self.slice = rest;

        for itemref in to_drop.iter() {
            let ptr = itemref as *const T as *mut T;
            /* SAFETY: Vec won't drop again this element (see VecDrain::new) */
            unsafe { ptr::drop_in_place(ptr) }
        }

        return self.next();
    }
}

impl<'a, T: 'a> DoubleEndedIterator for VecDrain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (last, rest) = self.slice.split_last()?;
        self.slice = rest;
        /* SAFETY: Vec won't drop the element and we only pass it once */
        let item = unsafe { ptr::read(last as *const T) };
        Some(item)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.slice.len();
        if n >= len {
            self.slice = &[];
            return None;
        }

        let n = len - n;
        let (rest, to_drop) = self.slice.split_at(n);
        self.slice = rest;

        for itemref in to_drop.iter() {
            let ptr = itemref as *const T as *mut T;
            /* SAFETY: Vec won't drop again this element (see VecDrain::new) */
            unsafe { ptr::drop_in_place(ptr) }
        }

        return self.next_back();
    }
}

