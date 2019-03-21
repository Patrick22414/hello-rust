fn main() {
    let ip4 = IpAddrKind::V4(127, 0, 0, 1);
    let ip6 = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", ip4);
    println!("{:?}", ip6);
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    Color(u8, u8, u8)
}
