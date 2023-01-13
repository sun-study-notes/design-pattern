// ===== 抽象层 =====

trait ICpu {
    fn calculate(&self);
}

trait IRam {
    fn storage(&self);
}

trait IGpu {
    fn display(&self);
}

// 抽象的工厂
trait IMakerFactory {
    fn create_cpu(&self) -> Box<dyn ICpu>;
    fn create_gpu(&self) -> Box<dyn IGpu>;
    fn create_ram(&self) -> Box<dyn IRam>;
}

// ===== 实现层 =====
struct IntelCpu {}
struct IntelRam {}
struct IntelGpu {}

impl ICpu for IntelCpu {
    fn calculate(&self) {
        println!("cpu is working")
    }
}
impl IGpu for IntelGpu {
    fn display(&self) {
        println!("gpu is working")
    }
}
impl IRam for IntelRam {
    fn storage(&self) {
        println!("Ram is working")
    }
}

struct IntelFactory {}

impl IMakerFactory for IntelFactory {
    fn create_cpu(&self) -> Box<dyn ICpu> {
        Box::new(IntelCpu {})
    }

    fn create_gpu(&self) -> Box<dyn IGpu> {
        Box::new(IntelGpu {})
    }

    fn create_ram(&self) -> Box<dyn IRam> {
        Box::new(IntelRam {})
    }
}
struct NvidiaCpu {}
struct NvidiaRam {}
struct NvidiaGpu {}

impl ICpu for NvidiaCpu {
    fn calculate(&self) {
        println!("cpu is working")
    }
}
impl IGpu for NvidiaGpu {
    fn display(&self) {
        println!("gpu is working")
    }
}
impl IRam for NvidiaRam {
    fn storage(&self) {
        println!("Ram is working")
    }
}

struct NvidiaFactory {}

impl IMakerFactory for NvidiaFactory {
    fn create_cpu(&self) -> Box<dyn ICpu> {
        Box::new(NvidiaCpu {})
    }

    fn create_gpu(&self) -> Box<dyn IGpu> {
        Box::new(NvidiaGpu {})
    }

    fn create_ram(&self) -> Box<dyn IRam> {
        Box::new(NvidiaRam {})
    }
}

struct KingstonCpu {}
struct KingstonRam {}
struct KingstonGpu {}

impl ICpu for KingstonCpu {
    fn calculate(&self) {
        println!("cpu is working")
    }
}
impl IGpu for KingstonGpu {
    fn display(&self) {
        println!("gpu is working")
    }
}
impl IRam for KingstonRam {
    fn storage(&self) {
        println!("KingstonRam is working")
    }
}

struct KingstonFactory {}

impl IMakerFactory for KingstonFactory {
    fn create_cpu(&self) -> Box<dyn ICpu> {
        Box::new(KingstonCpu {})
    }

    fn create_gpu(&self) -> Box<dyn IGpu> {
        Box::new(KingstonGpu {})
    }

    fn create_ram(&self) -> Box<dyn IRam> {
        Box::new(KingstonRam {})
    }
}

struct Computer {
    cpu: Box<dyn ICpu>,
    gpu: Box<dyn IGpu>,
    ram: Box<dyn IRam>,
}
impl Computer {
    pub fn new(cpu: Box<dyn ICpu>, gpu: Box<dyn IGpu>, ram: Box<dyn IRam>) -> Computer {
        Computer { cpu, gpu, ram }
    }
}

fn main() {
    // 要求组装两台电脑，
    // 1台（Intel的CPU，Intel的显卡，Intel的内存）
    // 1台（Intel的CPU， nvidia的显卡，Kingston的内存）
    let intel_factory = IntelFactory {};
    let cpu = intel_factory.create_cpu();
    let gpu = intel_factory.create_gpu();
    let ram = intel_factory.create_ram();

    // 1台（Intel的CPU，Intel的显卡，Intel的内存）
    let computer = Computer::new(cpu, gpu, ram);
    computer.gpu.display();
    computer.cpu.calculate();
    computer.ram.storage();

    let intel_factory = NvidiaFactory {};

    // 1台（Intel的CPU， nvidia的显卡，Kingston的内存）
    let computer = Computer::new(
        intel_factory.create_cpu(),
        NvidiaFactory {}.create_gpu(),
        KingstonFactory {}.create_ram(),
    );
    computer.gpu.display();
    computer.cpu.calculate();
    computer.ram.storage();
}
