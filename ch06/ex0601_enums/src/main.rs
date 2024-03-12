/// enumerations are also referred to as enums
/// enums allow you to define a type by enumerating its possible variants

/// enums give you a way of saying a value is one of a possible set of values
/// example where an enum is more appropriate than a struct:
/// enumerating all possible variants of an IP address
#[derive(Debug)]
enum IpAddrKind {
    V4, 
    V6,
} // IpAddrKind is now a custom data type

/// a first approach to store the actual IP address data
#[derive(Debug)]
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

fn main() {
    println!("------------------------------------------------------------");
    println!("Defining an Enum");
    println!("------------------------------------------------------------");
    // creating instances of each of the two variants of IpAddrKind
    // the variants of the enum are namespaced under its identifier
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // instances can be used in a function that takes any kind of IP address
    let _route_v4 = route(four);
    let _route_v6 = route(six);

    // a first approach to store the actual IP address data using a struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    println!("{:?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };
    println!("{:?}", loopback);


}

fn route(_ip_address: IpAddrKind) {}