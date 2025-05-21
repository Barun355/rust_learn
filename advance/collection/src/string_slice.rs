

pub fn main() {

    // Strings
    let mut name = String::from("Barun");

    name.push_str(" Tiwary");

    name.replace_range(5..name.len(), " Software Engineer");

    println!("{}", name);

    // Slice let you reference the contiguous sequence of elements in a collection rather than the whole collection. A Slice is a kind of reference, so it does not have ownership.

    let word = String::from("Hello Tiwary");

    let word2 = &word[0..5];

    // word.replace_range(0..5, "Barun ");

    println!("{}", word2);

}

pub fn first_word(sentence: &String) -> &str {
    let mut index = 0;

    for (idx, c) in sentence.chars().enumerate() {
        if c == ' ' {
            index = idx;
            break;
        }
    }

    return &sentence[0..index];
}