#[cfg(test)]
mod tests {
    use crate::lexer::lexer::*;
    use crate::lexer::Token;

    #[test]
    fn test_matching_tokens(){
        let rules = alloc_rules();
        assert_eq!(get_matching_tokens("#",rules)[0],Token::Func)
    }
}
