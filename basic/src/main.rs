mod borrowing;


fn find_pallindrome(word: String) -> i8 {

    return 0;
}


fn main() {
    // let word: String = String::from("abccba");
    // println!("{}", find_pallindrome(word))

    let mut my = String::from("hello");

    borrowing::borrow(&mut my);

    println!("{}", my)

}
