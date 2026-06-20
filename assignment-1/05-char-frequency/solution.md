# Solution notes: Character frequency, sorted

## Approach

I used a `HashMap` to count the frequency of each character in the input string. Then, I converted the map into a vector of tuples and sorted it by frequency (descending) and then alphabetically.

## Edge cases handled

- Empty input string
- Strings with all unique characters
- Strings with all identical characters

## Anything special

sorted the output by frequency and then alphabetically for characters with the same frequency.
