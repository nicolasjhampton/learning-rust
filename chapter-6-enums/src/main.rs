
// enum IPAddrKind {
//     V4,
//     V6,
// }

// struct IPAddr {
//     kind: IPAddrKind,
//     address: String,
// }

// standard library implementation
// struct Ipv4Addr {
//     // details elided
// }
// struct Ipv6Addr {
//     // details elided
// }
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}



#[derive(Debug)]
enum UsState {
    Michigan,
    Oklahoma,
    Florida,
    Oregon,
    Illinois,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state is {:?}.", state);
            25
        },
    }
}

fn main() {
    // let four = IPAddrKind::V4;
    // let six = IPAddrKind::V6;
    // let home = &IPAddr{
    //     kind: IPAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = &IPAddr{
    //     kind: IPAddrKind::V6,
    //     address: String::from("::1"),
    // };
    // route(four);
    // route(six);
    let home = IPAddr::V4(127 ,0, 0, 1);
    let loopback = IPAddr::V6(String::from("::1"));
    println!("An IP address enum! {:?}", home);
    println!("Another IP address enum! {:?}", loopback);
    value_in_cents(Coin::Quarter(UsState::Michigan));
}

// fn route(ip_type: IPAddrKind) {

// }
