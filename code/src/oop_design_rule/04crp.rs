struct Cat {}
struct CatB {}

impl Cat {
    fn eat(&self) {
        println!("cat is eating")
    }
}

impl CatB {
    fn eat(&self, cat: &Cat) {
        cat.eat();
    }
    fn sleep(&self) {
        println!("cat is sleeping")
    }
}

fn main() {
    let cat = &Cat {};
    cat.eat();

    let catb = &CatB {};
    catb.eat(cat);
    catb.sleep();
}
