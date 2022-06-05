// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     }

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     }
// }

// fn route(ip_kind: IpAddrKind) {

// }


// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(String::from(127, 0, 0, 1));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// OPTION

// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a String");

//     let absent_number: Option<i32> = None;
// }

// MATCH

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin:: Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
//     let val = value_in_cents(Coin::Quarter(UsState::Alaska));
//     let word = if val == 1 {
//         "cent"
//     } else {
//         "cents"
//     };
//     println!("I have {} {}", val, word);
// }


// Matching with Option<T>

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }


// If Let

fn main() {
    let some_u8_value = None;
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("nope");
    }
}