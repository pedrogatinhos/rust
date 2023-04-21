struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ active: {}, username: {}, email: {}, sign_in_count: {} }}",
            self.active, self.username, self.email, self.sign_in_count
        )
    }
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    print!("User {{ active: val, username: val, email: val, sign_in_count: val }}");
    println!("{}", user1)
}
