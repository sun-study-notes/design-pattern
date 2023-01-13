// ===== 抽象层 =====
// 水果
trait IFruit {
    fn show(&self);
}
// 水果工厂
trait IFruitFactory {
    fn create_fruit(&self) -> &dyn IFruit;
}

// ===== 实现层 =====
struct Apple {}

impl IFruit for Apple {
    fn show(&self) {
        println!("我是苹果")
    }
}
struct Banner {}

impl IFruit for Banner {
    fn show(&self) {
        println!("我是香蕉")
    }
}

struct AppleFactory {}

impl IFruitFactory for AppleFactory {
    fn create_fruit(&self) -> &dyn IFruit {
        &Apple {}
    }
}
struct BannerFactory {}

impl IFruitFactory for BannerFactory {
    fn create_fruit(&self) -> &dyn IFruit {
        &Banner {}
    }
}

// ===== 业务逻辑层 =====
fn main() {
    let apple_factory = &AppleFactory {};
    let apple = apple_factory.create_fruit();
    apple.show();

    let banner_factory = &BannerFactory {};
    banner_factory.create_fruit().show();
}
