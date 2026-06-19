# Solution notes: Censor vowels in place

## Approach

First will split the string into a list of characters, then iterate over them and replace any vowels either lowercase or uppercase with an asterisk. Finally, will join the list back into a string and return it.

## Edge cases handled

- Empty string: The function will return an empty String if the input is empty.
- String with no vowels: The function will return the original string if it contains no vowels.
- String with all vowels: The function will return a string of asterisks if the input contains only vowels.

## Anything special
