use std::collections::HashMap;

fn main() {
    const CHARACTERS: [char; 36] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    ];

    const MORSE_CODES: [&str; 36] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
        "-----",
    ];

    let mut char_to_code_map: HashMap<char, &str> = HashMap::new();
    let mut code_to_char_map: HashMap<&str, char> = HashMap::new();

    for i in 0..CHARACTERS.len() {
        char_to_code_map.insert(CHARACTERS[i], MORSE_CODES[i]);
        code_to_char_map.insert(MORSE_CODES[i], CHARACTERS[i]);
    }

    let char_result = translate_char_to_code(char_to_code_map, 'A');
    let morse_result = translate_code_to_char(code_to_char_map, ".");

    println!("{}", char_result); // prints '.-
    println!("{}", morse_result); // prints 'E'
}

fn translate_char_to_code(dictionary: HashMap<char, &str>, input: char) -> &str {
    match dictionary.get(&input) {
        Some(&result) => &result,
        _ => "",
    }
}

fn translate_code_to_char(dictionary: HashMap<&str, char>, input: &str) -> char {
    match dictionary.get(&input) {
        Some(char) => *char,
        _ => '\0',
    }
}
