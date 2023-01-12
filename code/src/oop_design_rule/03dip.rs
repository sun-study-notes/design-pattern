// 抽象层

// 司机
trait Driver {
    fn drive<T>(&self, car: &T)
    where
        T: Car;
}
// 汽车
trait Car {
    fn run(&self);
}

// 实现层
struct Zhangsan {}

impl Driver for Zhangsan {
    fn drive<T>(&self, car: &T)
    where
        T: Car,
    {
        println!("Zhangsan 开车");
        car.run();
    }
}
struct Lisi {}

impl Driver for Lisi {
    fn drive<T>(&self, car: &T)
    where
        T: Car,
    {
        println!("Lisi 开车");
        car.run();
    }
}

struct BenZ {}

impl Car for BenZ {
    fn run(&self) {
        println!("BenZ is running")
    }
}
struct Bmw {}

impl Car for Bmw {
    fn run(&self) {
        println!("Bmw is running")
    }
}

fn main() {
    let benz = &BenZ {};
    let bmw = &Bmw {};
    let zhangsan = &Zhangsan {};
    zhangsan.drive(bmw);

    let lisi = &Lisi {};
    lisi.drive(benz);

    lisi.drive(bmw);
}
