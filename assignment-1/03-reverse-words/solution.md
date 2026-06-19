# Solution notes: Reverse the word order

## Approach

Take the string, split it into words. Loop in reverse the list of words, and then join them back together with spaces.

## Edge cases handled

- Empty string: The split will return an empty list, and the join will return an empty string.
- String with whitespaces: The split handle tabs and next lines as well, so it will work correctly.

## Anything special

The last space is need to be removed.
