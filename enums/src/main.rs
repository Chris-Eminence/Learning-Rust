fn main() {
    
    struct Spaceship {
        name: String,
        color: String,
        engines: i32,
    }

    let my_spaceship = Spaceship {
        name: String::from("Star Blaster"),
        color: String::from("red"),
        engines: 2,
    };

    println!("Hello, world! {}", my_spaceship.name);


    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

}

