fn main() {
    // more on structs
    // let user1 = User::create_user(
    //     1,
    //     String::from("poop_master3000"),
    //     String::from("poop"),
    //     String::from("master"),
    //     19690420 // YYYYMMDD
    // );

    // let black = HexaColor(0, 0, 0);

    // println!("{:?}", user1); // debug mode only
#[derive(Debug)]
    enum IpAddKind { // you can use an enum as a type in a struct or as type in general.
        V4(String),
        V6(String), // you can declare them as string or anything else to define the data type
    }

    // YOU CAN USE STRUCTS:
    struct IpAddr {
        ip: IpAddKind,
        name: String,
    }

    let ip_v4_add_kind = IpAddKind::V4;

    let ip_address: IpAddr = {
        IpAddr {
            ip: ip_v4_add_kind,
            name: String::from("IPV4"),
        }
    };

    // OR SIMPLY JUST CALL IT DIRECTLY AFTER SETTING THE DATA TYPE:
    
    let I_P_V_4 = IpAddKind::V4(String::from("0.0.0.0"));
    println!("{:?}", I_P_V_4);

    // kinda like const ip = { kind: "V4", value: "0.0.0.0" }; in js
}

// #[derive(Debug)] // you need this to :? print a struct. only use in development
// // without it youll get "cannot be formatted using `{:?}` because it doesn't implement `Debug`"
// struct User {
//     id: i64,
//     user_name: String,
//     first_name: String,
//     last_name: String,
//     dob: i32,
// }

// impl User {
//     fn create_user(
//         id: i64,
//         user_name: String,
//         first_name: String,
//         last_name: String,
//         dob: i32
//     ) -> User {
//         User {
//             id,
//             user_name: user_name,
//             first_name,
//             last_name,
//             dob,
//         }
//     }
// }

// // tuple-like struct
// struct HexaColor(i32, i32, i32);
