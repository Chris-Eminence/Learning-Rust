fn main() {
    // Print a greeting message to the console
    println!("Hello, world!");

    // Define an enum to represent the kind of IP address (either V4 or V6)
    enum IpAddrKind {
        V4,
        V6,
    }

    // Define a struct to store an IP address along with its kind
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    // Create an instance of IpAddr for an IPv4 address
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // Create an instance of IpAddr for an IPv6 address
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Define a generic enum to represent an optional value
    enum Option<T> {
        None,
        Some(T),
    }

    // Create instances of Option with some values
    // The type of the value is inferred from the value itself
    let some_number = Some(5);
    let some_char = Some('e');

    // Create instances of Option with no values (None)
    let absent_number: Option<i32> = None;
    let absent_char: Option<char> = None;
}
