#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use std::mem;
    use crate::lexer::lexer::*;
    use crate::lexer::{Token, TokenRule};

    fn check_single_token_parse(string:String, rules:&Vec<Box<dyn TokenRule>>, token: Token) -> bool{
        let t = get_matching_tokens(string,rules);
        dbg!(&t);
        if t.len()!=1 {return false;}
        mem::discriminant(&t[0]) == mem::discriminant(&token)
    }


    #[test]
    fn test_matching_literal_tokens(){
        let rules = alloc_rules();
        assert!(check_single_token_parse("#".to_string(),&rules,Token::Func));
        assert!(check_single_token_parse("@".to_string(),&rules,Token::Loop));
        assert!(check_single_token_parse(":".to_string(),&rules,Token::TypeSeparator));
        assert!(check_single_token_parse("..".to_string(),&rules,Token::RangeSeparator));
        assert!(check_single_token_parse("(".to_string(),&rules,Token::OpenParam));
        assert!(check_single_token_parse(")".to_string(),&rules,Token::CloseParam));
        assert!(check_single_token_parse("{".to_string(),&rules,Token::OpenBracket));
        assert!(check_single_token_parse("}".to_string(),&rules,Token::CloseBracket));
    }
}
