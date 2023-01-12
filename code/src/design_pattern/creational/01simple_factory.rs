trait Fruit {
    fn show(&self);
}

struct Apple {}

impl Fruit for Apple {
    fn show(&self) {
        println!("我是苹果")
    }
}
struct Banner {}

impl Fruit for Banner {
    fn show(&self) {
        println!("我是香蕉")
    }
}

enum FruitT {
    Apple,
    Banner,
}
struct Factory {}
impl Factory {
    fn create(name: FruitT) -> Box<dyn Fruit> {
        match name {
            FruitT::Apple => Box::new(Apple {}),
            FruitT::Banner => Box::new(Banner {}),
        }
    }
}

fn main() {
    Factory::create(FruitT::Apple).show();
    Factory::create(FruitT::Banner).show();
}
