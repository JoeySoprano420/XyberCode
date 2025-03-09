#[derive(Debug, Clone)]
enum Value {
    Int(i32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
struct Variable {
    name: String,
    value: Value,
}

fn set_variable(variables: &mut HashMap<String, Variable>, name: String, value: Value) {
    variables.insert(name.clone(), Variable { name, value });
}

fn get_variable(variables: &mut HashMap<String, Variable>, name: &str) -> Option<&Variable> {
    variables.get(name)
}
