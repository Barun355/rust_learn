pub trait Summary {
    fn summarise(&self) -> String;
}

pub struct User {
    name: String,
    age: i32,
}

impl Summary for User {
    fn summarise(&self) -> String {
        return format!("The user name is {} and there age is {}", self.name, self.age);
    }
}

fn main() {
    
    let user = User {
        name: String::from("Barun Tiwary"),
        age: 21
    };
    
    // println!("{}", user.summarise());
    
    notify(&user);
    notify_trait_bound(&user);
}


// user: &impl Summary is the syntatic sugar for the trait bound which is implemented in below f.
pub fn notify(user: &impl Summary) {
    println!("Notify user about the summary: {}", user.summarise());
}

pub fn notify_trait_bound<T: Summary>(user: &T) {
    println!("Notify user about the summary using trait bound : {}", user.summarise());
}