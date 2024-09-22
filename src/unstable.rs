// This file contains unstable APIs from the Rust standard library, which are subject to change.

use std::ptr;

/// Moves all but the first of consecutive elements to the end of the slice satisfying
/// a given equality relation.
///
/// Returns two slices. The first contains no consecutive repeated elements.
/// The second contains all the duplicates in no specified order.
///
/// The `same_bucket` function is passed references to two elements from the slice and
/// must determine if the elements compare equal. The elements are passed in opposite order
/// from their order in the slice, so if `same_bucket(a, b)` returns `true`, `a` is moved
/// at the end of the slice.
///
/// If the slice is sorted, the first returned slice contains no duplicates.
///
/// # Examples
///
/// ```
/// #![feature(slice_partition_dedup)]
///
/// let mut slice = ["foo", "Foo", "BAZ", "Bar", "bar", "baz", "BAZ"];
///
/// let (dedup, duplicates) = slice.partition_dedup_by(|a, b| a.eq_ignore_ascii_case(b));
///
/// assert_eq!(dedup, ["foo", "BAZ", "Bar", "baz"]);
/// assert_eq!(duplicates, ["bar", "Foo", "BAZ"]);
/// ```
#[inline]
pub fn partition_dedup_by<T, F>(sl: &mut [T], mut same_bucket: F) -> (&mut [T], &mut [T])
where
  F: FnMut(&mut T, &mut T) -> bool,
{
  // Although we have a mutable reference to `sl`, we cannot make
  // *arbitrary* changes. The `same_bucket` calls could panic, so we
  // must ensure that the slice is in a valid state at all times.
  //
  // The way that we handle this is by using swaps; we iterate
  // over all the elements, swapping as we go so that at the end
  // the elements we wish to keep are in the front, and those we
  // wish to reject are at the back. We can then split the slice.
  // This operation is still `O(n)`.
  //
  // Example: We start in this state, where `r` represents "next
  // read" and `w` represents "next_write".
  //
  //           r
  //     +---+---+---+---+---+---+
  //     | 0 | 1 | 1 | 2 | 3 | 3 |
  //     +---+---+---+---+---+---+
  //           w
  //
  // Comparing sl[r] against sl[w-1], this is not a duplicate, so
  // we swap sl[r] and sl[w] (no effect as r==w) and then increment both
  // r and w, leaving us with:
  //
  //               r
  //     +---+---+---+---+---+---+
  //     | 0 | 1 | 1 | 2 | 3 | 3 |
  //     +---+---+---+---+---+---+
  //               w
  //
  // Comparing sl[r] against sl[w-1], this value is a duplicate,
  // so we increment `r` but leave everything else unchanged:
  //
  //                   r
  //     +---+---+---+---+---+---+
  //     | 0 | 1 | 1 | 2 | 3 | 3 |
  //     +---+---+---+---+---+---+
  //               w
  //
  // Comparing sl[r] against sl[w-1], this is not a duplicate,
  // so swap sl[r] and sl[w] and advance r and w:
  //
  //                       r
  //     +---+---+---+---+---+---+
  //     | 0 | 1 | 2 | 1 | 3 | 3 |
  //     +---+---+---+---+---+---+
  //                   w
  //
  // Not a duplicate, repeat:
  //
  //                           r
  //     +---+---+---+---+---+---+
  //     | 0 | 1 | 2 | 3 | 1 | 3 |
  //     +---+---+---+---+---+---+
  //                       w
  //
  // Duplicate, advance r. End of slice. Split at w.

  let len = sl.len();
  if len <= 1 {
    return (sl, &mut []);
  }

  let ptr = sl.as_mut_ptr();
  let mut next_read: usize = 1;
  let mut next_write: usize = 1;

  // SAFETY: the `while` condition guarantees `next_read` and `next_write`
  // are less than `len`, thus are inside `sl`. `prev_ptr_write` points to
  // one element before `ptr_write`, but `next_write` starts at 1, so
  // `prev_ptr_write` is never less than 0 and is inside the slice.
  // This fulfils the requirements for dereferencing `ptr_read`, `prev_ptr_write`
  // and `ptr_write`, and for using `ptr.add(next_read)`, `ptr.add(next_write - 1)`
  // and `prev_ptr_write.offset(1)`.
  //
  // `next_write` is also incremented at most once per loop at most meaning
  // no element is skipped when it may need to be swapped.
  //
  // `ptr_read` and `prev_ptr_write` never point to the same element. This
  // is required for `&mut *ptr_read`, `&mut *prev_ptr_write` to be safe.
  // The explanation is simply that `next_read >= next_write` is always true,
  // thus `next_read > next_write - 1` is too.
  unsafe {
    // Avoid bounds checks by using raw pointers.
    while next_read < len {
      let ptr_read = ptr.add(next_read);
      let prev_ptr_write = ptr.add(next_write - 1);
      if !same_bucket(&mut *ptr_read, &mut *prev_ptr_write) {
        if next_read != next_write {
          let ptr_write = prev_ptr_write.add(1);
          ptr::swap(ptr_read, ptr_write);
        }
        next_write += 1;
      }
      next_read += 1;
    }
  }

  sl.split_at_mut(next_write)
}
