use wasmer_runtime::{imports, Instance};

fn execute_wasm(wasm_bytes: Vec<u8>) {
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes).expect("Failed to compile WASM");

    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).expect("Failed to instantiate WASM");

    let result = instance
        .exports
        .get_function("main")
        .expect("Function not found")
        .call(&[])
        .expect("Failed to call function");

    println!("Execution result: {:?}", result);
}
