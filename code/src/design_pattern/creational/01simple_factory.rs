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

struct Factory {}
impl Factory {
    fn create(name: String) -> Box<dyn Fruit> {
        let _apple = "apple".to_string();
        match name {
            _apple => Box::new(Apple {}),
            _ => Box::new(Apple {}),
        }
    }
}

fn main() {
    Factory::create("apple".to_string()).show();
}
