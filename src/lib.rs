mod slice_iter;
pub use slice_iter::SliceIter;

mod slice_iter_mut;
pub use slice_iter_mut::SliceIterMut;

mod vec_drain;
pub use vec_drain::VecDrain;

/// ```rust
/// # use safer_std::*;
/// let mut arr = [1u8, 2, 3, 4, 5, 6, 7];
/// rotate_right(&mut arr, 3);
/// assert_eq!(arr, [5u8, 6, 7, 1, 2, 3, 4]);
/// ```
pub fn rotate_right<T>(slice: &mut [T], k: usize) {
    assert!(k <= slice.len());
    reverse(slice);
    let (front, back) = slice.split_at_mut(k);
    reverse(front);
    reverse(back);
}

/// ```rust
/// # use safer_std::*;
/// let mut arr = [1u8, 2, 3, 4, 5, 6, 7];
/// rotate_left(&mut arr, 4);
/// assert_eq!(arr, [5u8, 6, 7, 1, 2, 3, 4]);
/// ```
pub fn rotate_left<T>(slice: &mut [T], k: usize) {
    let k = slice.len() - k;
    rotate_right(slice, k)
}

/// ```rust
/// # use safer_std::*;
/// let mut arr = [1u8, 2, 3, 4, 5, 6, 7];
/// reverse(&mut arr);
/// assert_eq!(arr, [7u8, 6, 5, 4, 3, 2, 1]);
/// ```
pub fn reverse<T>(slice: &mut [T]) {
    let mut iter = SliceIterMut::new(slice);
    while let Some(a) = iter.next() {
        if let Some(b) = iter.next_back() {
            core::mem::swap(a, b);
        }
    }
}

