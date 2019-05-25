use ascii::ToAsciiChar;

#[derive(Debug, PartialEq)]
enum Mode {
    ToUpper = 0,
    ToLower,
    Punctuation,
}

impl Mode {
    fn change_mode(&self) -> Self {
        match *self {
            Mode::ToUpper => Mode::ToLower,
            Mode::ToLower => Mode::Punctuation,
            Mode::Punctuation => Mode::ToUpper,
        }
    }
}

fn main() {
    return;
}

fn uppercase_mode(code_digit: u32) -> ascii::AsciiChar {
    let digit = code_digit % 27;
    let value = ToAsciiChar::to_ascii_char((digit as u8 % 27) + 64).unwrap();
    return value;
}

fn lowercase_mode(code_digit: u32) -> ascii::AsciiChar {
    let digit = code_digit % 27;
    let value = ToAsciiChar::to_ascii_char((digit as u8 % 27) + 96).unwrap();
    return value;
}

fn punctuation_mode(code_digit: u32) -> ascii::AsciiChar {
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
    let output: u8;

    println!("{}", input);
    match input {
        1 => output = 033,
        2 => output = 063,
        3 => output = 044,
        4 => output = 046,
        5 => output = 032,
        6 => output = 059,
        7 => output = 034,
        8 => output = 039,
        _ => panic!("Failed to parse {} . . .", input),
    }

    return ToAsciiChar::to_ascii_char(output as u8).unwrap();
}

fn decode(input: String) -> String {
    let mut major_mode: Mode = Mode::ToUpper;
    let mut decoded_message: String = "".to_string();
    let input_vec: Vec<u32> = input
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    for x in input_vec.iter() {
        if x % 27 == 0 || (major_mode == Mode::Punctuation && x % 9 == 0) {
            major_mode = major_mode.change_mode();
        } else {
            match major_mode {
                Mode::ToUpper => decoded_message.push(uppercase_mode(*x).as_char()),
                Mode::ToLower => decoded_message.push(lowercase_mode(*x).as_char()),
                Mode::Punctuation => decoded_message.push(punctuation_mode(*x).as_char()),
            }
        }
    }
    return decoded_message;
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

    #[test]
    fn decode_test() {
        let x = "18,12312,171,763,98423,1208,216,11,500,18,241,0,32,20620,27,10".to_string();
        assert_eq!("Right? Yes!", decode(x));
    }
}
