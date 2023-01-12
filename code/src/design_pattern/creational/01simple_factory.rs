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
struct FruitFactory {}
impl FruitFactory {
    fn create(fruit_t: &FruitT) -> Box<dyn Fruit> {
        match fruit_t {
            FruitT::Apple => Box::new(Apple {}),
            FruitT::Banner => Box::new(Banner {}),
        }
    }
}

fn main() {
    FruitFactory::create(&FruitT::Apple).show();
    FruitFactory::create(&FruitT::Banner).show();
}
