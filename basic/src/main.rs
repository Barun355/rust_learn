mod borrowing;
mod enums;

fn find_pallindrome(_word: String) -> i8 {

    return 0;
}


fn main() {
    // let word: String = String::from("abccba");
    // println!("{}", find_pallindrome(word))

    // let mut my = String::from("hello");

    // borrowing::borrow(&mut my);

    // println!("{}", my)
    //
    let user: User = create_obj();
    println!("User {} has email {} and there login count is {}", user.name, user.email, user.login_count );

    enums::start_enum();

}

struct User {                                                       name: String,                                                   email: String,
    login_count: i64,                                               active: bool                                                }

fn create_obj() -> User {
    let user = User {
        name: String::from("Pawan Sharma"),
        email: String::from("pawan@gmail.com"),
        login_count: 10,
        active: false
    };

    return user;
}
