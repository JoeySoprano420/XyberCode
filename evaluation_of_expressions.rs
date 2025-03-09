fn evaluate_expression(expr: &Expr, variables: &mut HashMap<String, i32>) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Variable(name) => *variables.get(name).unwrap_or(&0),
        Expr::Add(left, right) => evaluate_expression(left, variables) + evaluate_expression(right, variables),
        Expr::Sub(left, right) => evaluate_expression(left, variables) - evaluate_expression(right, variables),
        _ => unimplemented!(),
    }
}
