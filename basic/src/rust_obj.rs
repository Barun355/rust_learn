

struct User {
    name: String,
    email: String,
    login_count: i64,
    active: bool
}

fn crrate_obj() -> User {
    let user = User {
        name: String::from("Pawan Sharma"),
        email: String::from("pawan@gmail.com"),
        login_count: 10,
        active: false
    }

    return user
}
