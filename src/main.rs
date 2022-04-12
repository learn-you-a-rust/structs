// Each field in a struct must have its
// type defined
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// An instance of a struct defines concrete
// values for each of the fields defined above
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username:String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
