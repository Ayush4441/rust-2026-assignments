# Solution notes: Split and double

## Approach
The solution uses the `split_at_mut` method to divide the vector into two mutable slices at the specified midpoint. It then doubles all elements in both slices.

## Edge cases handled
- `mid == 0`: left slice is empty, doubling loop is a no-op, right slice covers the whole vector.
- `mid == xs.len()`: right slice is empty, left slice covers the whole vector.
- Empty vector with `mid == 0`: both slices are empty, both loops no-op.
- `mid > xs.len()`: `split_at_mut` panics on its own, satisfying the required panic behavior without any manual bounds check.

## Anything special
The whole exercise hinges on `split_at_mut`: it's the one method that lets you carve a single `&mut [i32]` into two non-overlapping `&mut [i32]` halves and hand both back to the caller. Doing this by hand (e.g. two separate `&mut xs[..mid]` and `&mut xs[mid..]` borrows) wouldn't compile, since the borrow checker can't prove on its own that the two ranges don't alias — `split_at_mut` proves it once, internally, so the rest of the function is just plain iteration and multiplication.