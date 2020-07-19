pub struct SliceIter<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> SliceIter<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        Self { slice }
    }

    pub fn as_slice(&self) -> &'a [T] {
        self.slice
    }
}

impl<'a, T: 'a> Iterator for SliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.slice.split_first()?;
        self.slice = rest;
        return Some(first);
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.slice.len();
        (n, Some(n))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.slice.len() {
            self.slice = &[];
            return None;
        }

        let back = &self.slice[n..];
        let (first, rest) = back.split_first()?;
        self.slice = rest;
        return Some(first);
    }
}

impl<'a, T: 'a> DoubleEndedIterator for SliceIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (last, rest) = self.slice.split_last()?;
        self.slice = rest;
        return Some(last);
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.slice.len();
        if n >= len {
            self.slice = &[];
            return None;
        }
        
        let n = len - n;
        self.slice = &self.slice[..n];
        return self.next_back();
    }
}

