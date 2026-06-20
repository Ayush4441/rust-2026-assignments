pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String 
{
    let i = (input, shift);
    let len = ALPHABET.len() as i32;
    let norm_shift = ((i.1 % len) + len) % len;

    let mut result = String::new();
    for c in i.0.chars() 
    {
        if c.is_ascii_alphabetic() 
        {
            let is_upper = c.is_ascii_uppercase();
            let lower = c.to_ascii_lowercase();

            // find position of this letter in ALPHABET
            let pos = ALPHABET.find(lower).unwrap() as i32;
            let new_pos = ((pos + norm_shift) % len) as usize;
            let new_char = ALPHABET.as_bytes()[new_pos] as char;

            result.push(if is_upper 
            {
                new_char.to_ascii_uppercase()
            } 
            else 
            {
                new_char
            });
        } 
        else 
        {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
