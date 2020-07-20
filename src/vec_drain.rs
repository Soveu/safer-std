use core::ops::{Bound, RangeBounds};
use core::ptr;
use core::mem;

pub struct VecDrain<'a, T: 'a> {
    slice: &'a mut [T],
}

impl<'a, T: 'a> VecDrain<'a, T> {
    pub fn new<R: RangeBounds<usize>>(vec: &'a mut Vec<T>, range: R) -> Self {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n+1,
        };
        let end = match range.end_bound() {
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
        let newlen = oldlen - len;

        /* SAFETY (set_len): we want to shorten the vec, because later in iteration
         * we will be ptr::read()ing it, effectively duplicating elements without
         * clone(). This can cause double-free when panicing.
         * SAFETY (get_unchecked): we just grab the slice that we shortened
         */
        let slice = unsafe { 
            vec.set_len(newlen); 
            vec.get_unchecked_mut(newlen .. oldlen)
        };

        Self { slice }
    }
}

impl<'a, T: 'a> Drop for VecDrain<'a, T> {
    fn drop(&mut self) {
        /* SAFETY: Vec won't drop again this element (see VecDrain::new) */
        unsafe { ptr::drop_in_place(self.slice) }
    }
}

impl<'a, T: 'a> Iterator for VecDrain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;

        /* SAFETY: Vec won't drop the element and we are only passing it once */
        let item = unsafe { ptr::read(first) };

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.slice.len();
        (n, Some(n))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let n = core::cmp::min(n, self.slice.len());

        let slice = mem::replace(&mut self.slice, &mut []);
        let (to_drop, rest) = slice.split_at_mut(n);
        self.slice = rest;

        /* SAFETY: Vec won't drop again this element (see VecDrain::new) */
        unsafe { ptr::drop_in_place(to_drop) };

        return self.next();
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T: 'a> DoubleEndedIterator for VecDrain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.slice, &mut []);
        let (last, rest) = slice.split_last_mut()?;
        self.slice = rest;

        /* SAFETY: Vec won't drop the element and we only pass it once */
        let item = unsafe { ptr::read(last) };

        Some(item)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.slice.len();
        let n = len - core::cmp::min(n, len);

        let slice = mem::replace(&mut self.slice, &mut []);
        let (rest, to_drop) = slice.split_at_mut(n);
        self.slice = rest;

        /* SAFETY: Vec won't drop again this element (see VecDrain::new) */
        unsafe { ptr::drop_in_place(to_drop) };

        return self.next_back();
    }
}

