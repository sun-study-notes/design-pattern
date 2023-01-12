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
    fn create<T: Fruit>(&self, name: String) -> T {
        
    }
}

fn main() {}
