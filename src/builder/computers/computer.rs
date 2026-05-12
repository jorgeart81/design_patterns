use std::fmt;

#[derive(Debug)]
pub struct Computer {
    cpu: String,
    ram: String,
    storage: String,
    gpu: Option<String>,
}

impl Computer {
    pub fn new(cpu: String, ram: String, storage: String, gpu: Option<String>) -> Self {
        Self {
            cpu,
            ram,
            storage,
            gpu,
        }
    }

    pub fn cpu(&self) -> &str {
        &self.cpu
    }
    pub fn ram(&self) -> &str {
        &self.ram
    }
    pub fn storage(&self) -> &str {
        &self.storage
    }
    pub fn gpu(&self) -> Option<&str> {
        self.gpu.as_deref()
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Computer:")?;
        writeln!(f, "  CPU: {}", self.cpu)?;
        writeln!(f, "  RAM: {}", self.ram)?;
        writeln!(f, "  Storage: {}", self.storage)?;
        writeln!(f, "  GPU: {}", self.gpu.as_deref().unwrap_or("None"))?;
        Ok(())
    }
}
