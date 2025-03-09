fn generate_while_loop(cond: Expr, body: Expr) {
    while evaluate_expression(&cond, &mut variables) != 0 {
        evaluate_expression(&body, &mut variables);
    }
}
