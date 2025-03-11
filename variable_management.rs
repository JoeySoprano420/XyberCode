#[derive(Debug, Clone)]
enum Value {
    Int(i32),
}

fn set_variable(variables: &mut HashMap<String, Value>, name: String, value: Value) {
    variables.insert(name.clone(), value);
}

fn get_variable(variables: &HashMap<String, Value>, name: &str) -> Option<&Value> {
    variables.get(name)
}
