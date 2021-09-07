fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    // println!("Hello, world!");
    let user2 = User {
        email: String::from("anotherexample@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}", user2.email)
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    //email: email
        username, //username: username
        active: true,
        sign_in_count: 1,
    }
}
