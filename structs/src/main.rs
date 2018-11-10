struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = build_user(String::from("user@example.com"), String::from("user123"));
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser123"),
        ..user1
    };
    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shortcut for email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}
