# Solution notes: Group anagrams

## Approach
For each word, build a signature by lowercasing it, sorting its characters, and rebuilding it as a string. Words that are anagrams of each other (case-insensitively) end up with identical signatures. Group words by signature in a `HashMap<String, Vec<String>>`, pushing each original word (with its original casing intact) into the vector for its signature. Finally, collect just the map's values into a `Vec<Vec<String>>`.

## Edge cases handled
- Empty input: the map stays empty, so collecting its values gives `[]`.
- Single word: forms its own group of one.
- No anagrams at all: every word gets a unique signature, so every group has exactly one word.
- All anagrams: every word maps to the same signature, so they all land in one group.
- Different lengths never merge: sorted-and-lowercased strings of different lengths can never be equal, so they naturally get different signatures.

## Anything special
Iterating `words` in order and always pushing (never reordering) onto each signature's vector is what keeps the **input order preserved within each group**, as required. Group order itself is left to whatever order the `HashMap` happens to yield, which the README says is fine.

I used `to_lowercase()` before sorting so the signature comparison is case-insensitive, while pushing the original `word.clone()` (not the lowercased version) so the output preserves each word's original casing.