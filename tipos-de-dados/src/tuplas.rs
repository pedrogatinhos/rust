#[derive(Debug)]
struct Objeto2d {
    width: u32,
    height: u32,
}

impl Objeto2d {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
git 

fn main() {
    println!("Hello, world!");
    let retangulo =  Objeto2d{
        width: 10,
        height: 10,
    };
    soma_area(&retangulo);
    println!("forma do objeto: {:?}", retangulo)
}

fn soma_area(objeto: &Objeto2d) -> u32{
    let area = objeto.width * objeto.height;
    println!("{area}");
    println!("{}", objeto.area());
    return area
}