use wasmer_runtime::{imports, Instance};

fn generate_wasm_code(expr: &Expr) -> Vec<u8> {
    let mut wasm_code = Vec::new();

    match expr {
        Expr::Number(n) => {
            wasm_code.push(0x01);  // Example byte for number
            wasm_code.push(*n as u8);
        }
        Expr::Add(left, right) => {
            let left_code = generate_wasm_code(left);
            let right_code = generate_wasm_code(right);
            wasm_code.extend(left_code);
            wasm_code.extend(right_code);
            wasm_code.push(0x02);  // Example byte for addition
        }
        _ => unimplemented!(),
    }

    wasm_code
}
