fn compile_aot(ast: &Vec<Statement>) -> Vec<u8> {
    let mut wasm_code = Vec::new();

    // Compile the AST directly to WebAssembly
    for statement in ast.iter() {
        wasm_code.extend(compile_statement(statement));
    }

    wasm_code
}
