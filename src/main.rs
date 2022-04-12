// Each field in a struct must have its
// type defined
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Unit-like structs don't have any fields,
// but can be used when you want to
// implement a trait on some type, but
// don't have data to store in a field
struct AlwaysEqual;

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

    // In a tuple struct, the fields don't have
    // names, just the types are defined
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
