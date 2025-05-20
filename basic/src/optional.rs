
pub fn find_first_occurence(sentence: &String, letter: char) -> Option<i32>  {
    
    for (i, c) in sentence.chars().enumerate() {
        if c == letter {
            return Some(i as i32);
        }
    }

    return None;
}