struct User {
    email: String,
    username: String,
    signins: u64,
    active: bool,
}

fn create_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        signins: 1,
    };
}

fn main() {
    // can define structs directly in functions (at least main())
    struct InFuncUser {
        email: String,
        username: String,
        signins: u64,
        active: bool,
    }

    let mut user0 = InFuncUser {
        email: String::from("asdf@asdf.asdf"),
        username: String::from("asdf"),
        active: false,
        signins: 345698,
    };

    println!(
        "{},{},{},{}",
        user0.email, user0.username, user0.active, user0.signins
    );

    let mut user1 = create_user(String::from("blah@blah.blah"), String::from("blah"));

    println!(
        "{},{},{},{}",
        user1.email, user1.username, user1.active, user1.signins
    );
}
