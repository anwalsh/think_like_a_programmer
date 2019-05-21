use ascii::ToAsciiChar;

enum Mode {
    ToUpper,
    ToLower,
    Punctuation,
}

fn main() {
    let x = 097;
    println!("{:?}", x.to_ascii_char());
    println!("{}", punctuation_mode(1));
    return;
}

fn uppercase_mode(code_digit: u32) -> ascii::AsciiChar {
    // TODO: input % 27 returns a integer associated with the nth character of the alphabet
    let digit = code_digit % 27;
    let value = ToAsciiChar::to_ascii_char((digit as u8 % 27) + 64).unwrap();
    return value;
}

fn lowercase_mode(code_digit: u32) -> ascii::AsciiChar {
    // TODO: the same as the above but with lowercase letters.
    let digit = code_digit % 27;
    let value = ToAsciiChar::to_ascii_char((digit as u8 % 27) + 96).unwrap();
    return value;
}

fn punctuation_mode(code_digit: u8) -> char {
    // Input % 9 returns an integer associated with a symbol:
    // 1=!
    // 2=?
    // 3=,
    // 4=.
    // 5=" "
    // 6=;
    // 7="
    // 8='
    let input = code_digit % 9;
    let mut output = ' ';

    match input {
        1 => output = '!',
        2 => output = '?',
        3 => output = ',',
        4 => output = '.',
        5 => output = ' ',
        6 => output = ';',
        7 => output = '"',
        8 => output = '\'',
        _ => panic!("Failed to parse {} . . .", input),
    }
    return output;
}

fn decode() {
    // TODO: main function to handle decoding a message. This function will switch on 0 to the
    // following mode.
    // User the Mode enum I am hoping to swap based on input.
}

fn parse_input() {
    // TODO: parse user input from the CLI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_mode_test() {
        let x = 18;
        assert_eq!('R', uppercase_mode(x));
        let y = 241;
        assert_eq!('Y', uppercase_mode(y));
    }

    #[test]
    fn lowercase_mode_test() {
        let x = 171;
        assert_eq!('i', lowercase_mode(x));
        let y = 20620;
        assert_eq!('s', lowercase_mode(y));
    }

    #[test]
    fn punctuation_mode_test() {
        let x = 1;
        assert_eq!('!', punctuation_mode(x));
        let y = 6;
        assert_eq!(';', punctuation_mode(y));
    }

    fn decode_test() {}

    fn parse_input_test() {}
}
