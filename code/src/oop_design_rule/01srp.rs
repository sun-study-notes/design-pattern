// 逛街时穿的衣服
struct ClothesShop {}

impl ClothesShop {
    fn style(&self) {
        println!("逛街时的装扮")
    }
}

// 工作时穿的衣服
struct ClothesWork {}

impl ClothesWork {
    fn style(&self) {
        println!("工作时的装扮")
    }
}

fn main() {
    let cs = &ClothesShop {};
    cs.style();

    let cw = &ClothesWork {};
    cw.style();
}
