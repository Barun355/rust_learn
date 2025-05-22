use std::fmt::Display;

struct User <'a> {
    name: &'a str,
}

fn main() {
    let longest_string;
    let first = String::from("First");

    {
        let second = String::from("second");
        longest_string = longest(&first, &second);
        println!("{}", longest_string);
    }

    struct_lifetime();
    let ans = longest_with_an_announcement("Hello", "world", "localhost");

    println!("{}", ans);
}

fn longest<'a>(first: &'a String, second: &'a String) -> &'a String {
    if first.len() > second.len() {
        return first;
    }

    return second;
}

fn struct_lifetime() {
    let name = "Barun Tiwary";
    let user = User { name };

    println!("{}", user.name);
}


fn longest_with_an_announcement<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str where T:Display{

    if a.len() > b.len() {
        return a;
    }

    println!("Ans: {ann}");

    return b;
}