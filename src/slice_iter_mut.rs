use core::mem;

pub struct SliceIterMut<'a, T: 'a> {
    slice: &'a mut [T],
}

impl<'a, T: 'a> SliceIterMut<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        Self { slice }
    }

    pub fn into_mut_slice(self) -> &'a mut [T] {
        self.slice
    }
}


impl<'a, T: 'a> Iterator for SliceIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        return Some(first);
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.slice.len();
        (n, Some(n))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.slice, &mut []);
        if n >= self.slice.len() {
            return None;
        }

        let (_, back) = slice.split_at_mut(n);
        let (first, rest) = back.split_first_mut()?;
        self.slice = rest;
        return Some(first);
    }
}

impl<'a, T: 'a> DoubleEndedIterator for SliceIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.slice, &mut []);
        let (last, rest) = slice.split_last_mut()?;
        self.slice = rest;
        return Some(last);
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.slice, &mut []);
        let len = slice.len();
        if n >= len {
            self.slice = &mut [];
            return None;
        }
        
        let n = len - n;
        self.slice = &mut slice[..n];
        return self.next_back();
    }
}

