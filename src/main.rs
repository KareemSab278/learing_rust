fn main() {
    // more on structs
    let user1 = User::create_user(
        1,
        String::from("poop_master3000"),
        String::from("poop"),
        String::from("master"),
        19690420 // YYYYMMDD
    );

    let black = HexaColor(0,0,0);

    println!("{:?}", user1); // debug mode only
}


#[derive(Debug)] // you need this to :? print a struct. only use in development
// without it youll get "cannot be formatted using `{:?}` because it doesn't implement `Debug`"
struct User {
    id: i64,
    user_name: String,
    first_name: String,
    last_name: String,
    dob: i32,
}

impl User {
    fn create_user(
        id: i64,
        user_name: String,
        first_name: String,
        last_name: String,
        dob: i32
    ) -> User {
        User {
            id,
            user_name: user_name,
            first_name,
            last_name,
            dob,
        }
    }
}

// tuple-like struct
struct HexaColor(i32, i32, i32);