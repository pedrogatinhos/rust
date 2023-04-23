#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    println!("{}","hello world" );

    let adress1 = IpAddr::V4(String::from("127.0.0.1"));
    let adress2 = IpAddr::V6(String::from("::1"));

    printKind(adress2);
    printKindV2(adress1);
}

fn printKind(value: IpAddr) {
    println!("versão 1:");
    println!("{:?}", value );
}

fn printKindV2(value: IpAddr) {
    println!("versão 2:");
    match value {
        IpAddr::V4(s) => println!("{:?}", s),
        IpAddr::V6(s) => println!("{:?}", s),
    }
}