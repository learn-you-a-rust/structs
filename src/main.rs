// each newly defined struct is its own type
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main {

    // to update the fields later, the entire instance must be 
    // mutable; you can't specify certain fields as mutable
    let mut user1 = User {
        // using Strings and not &strs allows the struct to retain
        // ownership of its fields' data;
        // you can only use non-owned types like &str if you use
        // "lifetimes"
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }    
 
    // we can update the email
    user1.email = String::from("anotheremail@example.com");

    // a new user instance is returned by making it the last
    // expression in a function
    fn build_user(email: String, username: String) -> User {
        User {
            // use field init shorthand
            email, //email: email,
            username, //username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    // create a new user based on the old one
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // fields not otherwise specified should be copied
        // from the below
        ..user1
    };

    // tuple structs have names for the struct, but not the fields;
    // these are different types since they are defined distinctly,
    // despite having the same types for the fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // these are different types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    

}

