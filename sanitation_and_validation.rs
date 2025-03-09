fn validate_ast(ast: &Vec<Statement>) -> bool {
    for statement in ast.iter() {
        match statement {
            Statement::Command(cmd) => validate_command(cmd),
            _ => return false,
        }
    }
    true
}

fn validate_command(cmd: &str) -> bool {
    !cmd.contains("unsafe")
}
