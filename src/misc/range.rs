use std::ops::RangeBounds;

pub fn range_bounds_to_range<R: RangeBounds<usize>>(
    r: R,
    start: usize,
    end: usize,
) -> (usize, usize) {
    use std::ops::Bound::*;

    let l = match r.start_bound() {
        Included(&l) => l,
        Excluded(&l) => l + 1,
        Unbounded => start,
    }
    .max(start);

    let r = match r.end_bound() {
        Included(&r) => r + 1,
        Excluded(&r) => r,
        Unbounded => end,
    }
    .min(end);

    (l, r)
}
