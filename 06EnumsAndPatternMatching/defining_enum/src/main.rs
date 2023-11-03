enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enums can be implemented
impl IpAddr {
    fn route(&self) {}
}

fn main() {
    // Assigning enum values to variables
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // calling functions which require enum
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using enum inside as a field of struct
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Creating enum which has stores value too
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
