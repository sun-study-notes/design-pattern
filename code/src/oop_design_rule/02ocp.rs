// 抽象的银行业务员
trait IBanker {
    fn do_busi(&self); //抽象的处理业务接口
}

// 存款业务员
struct SaveBanker {}

impl IBanker for SaveBanker {
    fn do_busi(&self) {
        println!("进行了存款")
    }
}

// 转账业务员
struct TransferBanker {}

impl IBanker for TransferBanker {
    fn do_busi(&self) {
        println!("进行了转账")
    }
}

// 实现一个框架层(基于抽象层进行业务封装-针对接口进行封装)
fn bank_business<T>(banker: &T)
where
    T: IBanker,
{
    // 通过接口向下来调(多态)
    banker.do_busi();
}

fn main() {
    let sb = &SaveBanker {};
    sb.do_busi();
    let tb = &TransferBanker {};
    tb.do_busi();

    bank_business(sb);
    bank_business(tb);
}
