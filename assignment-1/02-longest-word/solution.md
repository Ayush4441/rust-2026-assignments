# Solution notes: Longest word slice

## Approach
Split the input string into words, and then find the longest word by comparing their lengths.

## Edge cases handled
- Empty string: The function will return an empty None since there are no words to compare.
- Multiple words of the same length: The function will return the first longest word it encounters.


## Anything special

- first_on_tie() -> it return the first longest word it encounters as the rest of the words are of the same size not longer than the others.
