# Solution notes: Run-length encode

## Approach
- First I took a refference of the input.
- Then I created an vector of tuple<char, u32>.
- Then I looped over the all the chars in the refference of string.
- Next I took the last element from the vector as mutable, compared it with the char from the iteration.
- If they match then I incremented the count of the char in the vector. 
-Otherwise I pushed a new tuple of char and count 1 to the vector.

## Edge cases handled
- If empty input, returns an empty vector.
- Single character, returns one run with count 1.
- All same characters, collapses into one run with the total count.
- All different characters, each character becomes its own run.
- Whitespace, treated like any other character, counted in runs.

## Anything special

_Tricks, alternatives you considered, performance notes, etc._
