fn generate_if_statement(cond: Expr, then_block: Expr, else_block: Expr) {
    // Evaluate condition
    let condition = evaluate_expression(&cond, &mut variables);

    if condition != 0 {
        evaluate_expression(&then_block, &mut variables);
    } else {
        evaluate_expression(&else_block, &mut variables);
    }
}
