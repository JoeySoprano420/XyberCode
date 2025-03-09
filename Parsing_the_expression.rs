fn parse_expr(tokens: Vec<Token>) -> Expr {
    // Implementation of expression parsing
    if tokens.len() == 1 {
        if let Token::Number(n) = tokens[0] {
            return Expr::Number(n);
        }
    }

    if tokens.len() == 3 {
        let left = parse_expr(vec![tokens[0].clone()]);
        let right = parse_expr(vec![tokens[2].clone()]);
        match tokens[1] {
            Token::Add => Expr::Add(Box::new(left), Box::new(right)),
            Token::Sub => Expr::Sub(Box::new(left), Box::new(right)),
            _ => unimplemented!(),
        }
    }

    unimplemented!()
}
