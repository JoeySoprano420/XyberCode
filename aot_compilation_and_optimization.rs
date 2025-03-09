fn optimize_ast(ast: Expr) -> Expr {
    match ast {
        Expr::Add(left, right) => {
            let optimized_left = optimize_ast(*left);
            let optimized_right = optimize_ast(*right);

            if let (Expr::Number(l), Expr::Number(r)) = (&optimized_left, &optimized_right) {
                return Expr::Number(l + r);
            }

            Expr::Add(Box::new(optimized_left), Box::new(optimized_right))
        }
        _ => ast,
    }
}
