struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PartialOrd + std::fmt::Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x < self.y {
            println!("{} is less than {}", self.x, self.y);
        } else if self.x > self.y {
            println!("{} is greater than {}", self.x, self.y);
        } else {
            println!("{} and {} are equal", self.x, self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(5, 10);
    pair.cmp_display();

    let n1 = vec![1,2,4,5];
    let n2 = vec![1,2,4,4,5,6];   

    let pair2 = Pair::new(n1, n2);  //nao implementa os traços especificados então a struct nao recebe display
    pair2.cmp_display();
}
