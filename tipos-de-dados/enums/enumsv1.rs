#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

fn main() {
    println!("{}","hello world" );

    let adress1 = IpAddrKind::V4;
    let adress2 = IpAddrKind::V6;

    printKind(adress2);
}

fn printKind(value: IpAddrKind) {
    println!("{:?}", value );
}