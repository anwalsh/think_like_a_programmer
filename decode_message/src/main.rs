use ascii::ToAsciiChar;

enum Mode {
    ToUpper,
    ToLower,
    Punctuation,
}

fn main() {
    let x = 097;
    println!("{:?}", x.to_ascii_char());
    return;
}

fn upper_case_mode(code_digit: u8) -> char {
    // TODO: input % 27 returns a integer associated with the nth character of the alphabet
    return 'a';
}

fn lower_case_mode(code_digit: u8) -> char {
    // TODO: the same as the above but with lowercase letters.
    return 'a';
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
        _ => panic!("Failed to parse!"),
    }
    return output;
}

fn decode() {
    // TODO: main function to handle decoding a message. This function will switch on 0 to the
    // following mode.
}

fn parse_input() {
    // TODO: parse user input from the CLI
}
