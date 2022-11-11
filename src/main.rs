use std::collections::HashMap;

fn main() {
    let mut char_to_code_map: HashMap<char, &str> = HashMap::new();
    let mut code_to_char_map: HashMap<&str, char> = HashMap::new();

    let characters: [char; 36] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    ];
    let morse_codes: [&str; 36] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
        "-----",
    ];

    for i in 0..characters.len() {
        char_to_code_map.insert(characters[i], morse_codes[i]);
        code_to_char_map.insert(morse_codes[i], characters[i]);
    }
}
