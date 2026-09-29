#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::ast::*;
    use crate::lexer::lexer::parse;

    #[test]
    fn test_token_parsing() {
        let parsed = parse(include_str!("../../lang/LexerText1.dte").to_string());
        println!("{:?}",parsed)
    }

}
