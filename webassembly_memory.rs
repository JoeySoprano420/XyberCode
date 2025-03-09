fn allocate_memory(size: usize) -> i32 {
    // Allocate memory and return the starting pointer
    wasm_memory_allocate(size)
}

fn deallocate_memory(ptr: i32) {
    // Deallocate memory
    wasm_memory_deallocate(ptr)
}
