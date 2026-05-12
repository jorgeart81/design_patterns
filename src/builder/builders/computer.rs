use crate::builder::computers::Computer;

#[derive(Default)]
pub struct ComputerBuilder {
    cpu: Option<String>,
    ram: Option<String>,
    storage: Option<String>,
    gpu: Option<String>,
}

impl ComputerBuilder {
    pub fn set_cpu(mut self, cpu: &str) -> Self {
        self.cpu = Some(cpu.to_string());
        self
    }

    pub fn set_ram(mut self, ram: &str) -> Self {
        self.ram = Some(ram.to_string());
        self
    }

    pub fn set_storage(mut self, storage: &str) -> Self {
        self.storage = Some(storage.to_string());
        self
    }

    pub fn set_gpu(mut self, gpu: &str) -> Self {
        self.gpu = Some(gpu.to_string());
        self
    }

    pub fn build(self) -> Computer {
        Computer::new(
            self.cpu.expect("Please, set a cpu"),
            self.ram.expect("Please, set a ram"),
            self.storage.expect("Please, set a storage"),
            self.gpu,
        )
    }
}
