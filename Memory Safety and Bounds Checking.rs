fn check_bounds(memory: &Memory, index: usize, size: usize) -> Result<(), String> {
    if index + size <= memory.memory.len() {
        Ok(())
    } else {
        Err("Memory access out of bounds".to_string())
    }
}
