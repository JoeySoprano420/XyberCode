struct Memory {
    memory: Vec<u8>,
    size: usize,  // Memory size in pages (64KiB each)
}

impl Memory {
    fn new(size: usize) -> Self {
        let size_in_bytes = size * 65536;  // Each page is 64KB
        Self {
            memory: vec![0; size_in_bytes],
            size,
        }
    }

    fn grow(&mut self, additional_pages: usize) -> Result<(), String> {
        let additional_size = additional_pages * 65536;
        if self.size + additional_pages > 65536 {  // Assuming max allowed pages is 64MiB
            return Err("Memory limit exceeded".to_string());
        }
        self.memory.resize(self.memory.len() + additional_size, 0);
        self.size += additional_pages;
        Ok(())
    }

    fn get(&self, index: usize) -> Option<u8> {
        if index < self.memory.len() {
            Some(self.memory[index])
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, value: u8) -> Result<(), String> {
        if index < self.memory.len() {
            self.memory[index] = value;
            Ok(())
        } else {
            Err("Memory access out of bounds".to_string())
        }
    }
}
