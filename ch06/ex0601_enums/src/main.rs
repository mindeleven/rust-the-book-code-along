/// enumerations are also referred to as enums
/// enums allow you to define a type by enumerating its possible variants

/// enums give you a way of saying a value is one of a possible set of values
/// example where an enum is more appropriate than a struct:
/// enumerating all possible variants of an IP address
/// data can be put directly into an enum
/// we can put any kind of data inside an enum variant: 
/// strings, numeric types, structs or even another enum
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrKind {
    // definition says that both variants will have associated String values
    // each variant can have different types and amounts of associated data
    V4(u8, u8, u8, u8), 
    V6(String),
} // IpAddrKind is now a custom data type

// example of an enum with a wide variety of types
// to achieve the same with structs we would have to define four different structs
#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// we’re also able to define methods on enums using impl
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

/* no longer needed now that the data gets stored with the enum
/// a first approach to store the actual IP address data
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}
*/

fn main() {
    println!("------------------------------------------------------------");
    println!("Defining an Enum");
    println!("------------------------------------------------------------");
    // creating instances of each of the two variants of IpAddrKind
    // the variants of the enum are namespaced under its identifier
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    // instances can be used in a function that takes any kind of IP address
    let _route_v4 = route(four);
    let _route_v6 = route(six);

   // a first approach to store the actual IP address data using a struct
    /* 
    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    */
    let home = IpAddrKind::V4(127, 0, 0, 1);
    println!("{:?}", home);
    /*
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };
    */
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("{:?}", loopback);
    
    // example for using the Message enum
    let m = Message::Write(String::from("time waits for no one"));
    m.call();

}

fn route(_ip_address: IpAddrKind) {}