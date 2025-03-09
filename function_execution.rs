fn call_function(func: &Function, args: Vec<i32>, variables: &mut HashMap<String, i32>) -> i32 {
    for (i, param) in func.params.iter().enumerate() {
        set_variable(variables, param.clone(), Value::Int(args[i]));
    }

    evaluate_expression(&func.body, variables)
}
