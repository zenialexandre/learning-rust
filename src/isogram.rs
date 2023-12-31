use crate::helper;

pub fn init_isogram() {
    let mut isogram_input = String::new();
    isogram_input = helper::get_user_input(isogram_input, "String to be checked".to_string());
    build_chars_vector(remove_dash(isogram_input));
}

fn remove_dash(isogram_input: String) -> String {
    return isogram_input.trim().to_string().replace("-", "");
}

fn build_chars_vector(isogram_input: String) {
    let mut chars_vector: Vec<char> = vec![];

    for char in isogram_input.chars() {
        chars_vector.push(char);
    }
    check_isogram_result(is_isogram(chars_vector));
}

fn is_isogram(chars_vector: Vec<char>) -> bool {
    let mut generic_chars_vector: Vec<char> = vec![];
    let mut generic_char: &char;

    for (i, i_element) in chars_vector.iter().enumerate() {
        generic_chars_vector = chars_vector.clone();
        generic_char = i_element;
        generic_chars_vector.remove(i);

        for generic_element in &generic_chars_vector {
            if generic_char == generic_element {
                return false;
            }
        }
    }
    return true;
}

fn check_isogram_result(isogram_result: bool) {
    match isogram_result {
        true => println!("It's an isogram!"),
        false => println!("Isn't an isogram!")
    }
}
