#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::ast::*;
    use crate::lexer::lexer::parse;

    #[test]
    fn test_expression_parse() {
        let parsed = parse(include_str!("../../lang/ASTExpressionTest1.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_expr();
        println!("{:#?}",node);
    }

    #[test]
    fn test_close_param_eq() {
        assert_eq!(true, TokenType::CloseParam== &Token::CloseParam)
    }

}
