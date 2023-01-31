/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (sum, ok_chars, ok_spaces) =
        code.chars()
            .rev()
            .enumerate()
            .fold((0, 0, 0), |(sum, ok_chars, ok_spaces), (i, char)| {
                if char.is_whitespace() {
                    (sum, ok_chars, ok_spaces + 1)
                } else if let Some(digit) = char.to_digit(10) {
                    let i = i - ok_spaces;
                    let new_digit = if i % 2 == 1 {
                        if digit * 2 > 9 {
                            digit * 2 - 9
                        } else {
                            digit * 2
                        }
                    } else {
                        digit
                    };
                    (sum + new_digit, ok_chars + 1, ok_spaces)
                } else {
                    (sum, ok_chars, ok_spaces)
                }
            });

    let total_len = ok_chars + ok_spaces;

    if ok_chars > 1 && total_len == code.len() {
        sum % 10 == 0
    } else {
        false
    }
}
