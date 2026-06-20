# Solution notes: Caesar cipher

## Approach

The cipher works by shifting each letter in the plaintext by a certain number of positions down the alphabet. The process is repeated for each letter in the plaintext to produce the ciphertext.
To implement the Caesar cipher, we can follow these steps:
1. Define a function that takes the plaintext and the shift value as input.
2. Iterate through each character in the plaintext.
3. For each character, check if it is an uppercase or lowercase letter.
4. If it is a letter, shift it by the specified number of positions, wrapping around the alphabet if necessary.
5. If it is not a letter, leave it unchanged.
6. Join the shifted characters together to form the final ciphertext.

## Edge cases handled

- If the shift value is negative, we can still shift the letters in the opposite direction.
- If the shift value is greater than 26, we can use the modulus operator to wrap the shift value around the alphabet.
- Non-alphabetic characters are not modified and are included in the output as they are.


## Anything special
- The implementation can be done in a way that is case-sensitive, meaning that uppercase letters will remain uppercase and lowercase letters will remain lowercase after the shift.
- The function can be designed to handle both positive and negative shift values, allowing for flexibility in the direction of the shift.
