/// enumerations are also referred to as enums
/// enums allow you to define a type by enumerating its possible variants

/// enums give you a way of saying a value is one of a possible set of values
/// example where an enum is more appropriate than a struct:
/// enumerating all possible variants of an IP address
enum IpAddrKind {
    V4, 
    V6,
} // IpAddrKind is now a custom data type

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
}

fn route(_ip_address: IpAddrKind) {}