# Solution notes: Inventory

## Approach
`restock` takes both vectors by value, so it owns them outright. It chains the two into one iterator, consumes each `(name, qty)` pair, and folds them into a `HashMap<String, u32>`, adding to the existing quantity when a name repeats. The map is then collected back into a `Vec`, in whatever order the map happens to produce.

`summary` only borrows (`&[(String, u32)]`), so it never takes ownership of the inventory. It counts items with `.len()` and sums quantities with `.iter().map(...).sum()`, then formats both into the required string.

## Edge cases handled
- Both lists empty: the map ends up empty, collecting to `[]`.
- One list empty: the non-empty list's items pass through the map unchanged, with quantities unaffected since there's nothing to merge.
- Duplicate names across the two lists: quantities add via `or_insert(0)` followed by `+=`.
- Empty inventory in `summary`: `len()` and `sum()` both naturally return `0`, giving `"0 items, 0 units"`.

## Anything special
The key trick is just argument types: `restock` takes `Vec<...>` (owned) while `summary` takes `&[...]` (borrowed). That's what makes `summary(&inv)` followed by `restock(inv, more)` compile — `summary`'s borrow ends before `inv` is moved into `restock`. If `summary` took `Vec<...>` by value instead, that ordering would fail to compile, since `inv` would already be consumed.

I used a `HashMap` for merging rather than nested loops over both vectors, since it's simpler and avoids O(n²) lookups, even though performance isn't really a concern at this scale.