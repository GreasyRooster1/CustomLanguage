#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::lexer::NumberLiteralAssumptions::Float;
    use crate::lexer::{
        CLOSE_BRACKET_LITERAL, CLOSE_PARAM_LITERAL, FUNC_LITERAL, LOOP_LITERAL,
        OPEN_BRACKET_LITERAL, OPEN_PARAM_LITERAL, RANGE_SEPERATOR_LITERAL, TYPE_SEPERATOR_LITERAL,
        lexer::*,
    };
    use crate::lexer::{TokenRule, TokenType};
    use rand::{RngExt, random, rng};
    use std::mem;

    fn check_single_token_parse(
        string: String,
        rules: &Vec<Box<dyn TokenRule>>,
        token: TokenType,
    ) -> bool {
        let t = get_matching_tokens(string, rules);
        dbg!(&t);
        if t.len() != 1 {
            return false;
        }
        mem::discriminant(&t[0]) == mem::discriminant(&token)
    }

    #[test]
    fn test_matching_literal_tokens() {
        let rules = alloc_rules();
        assert!(check_single_token_parse(
            FUNC_LITERAL.to_string(),
            &rules,
            TokenType::Func
        ));
        assert!(check_single_token_parse(
            LOOP_LITERAL.to_string(),
            &rules,
            TokenType::Loop
        ));
        assert!(check_single_token_parse(
            TYPE_SEPERATOR_LITERAL.to_string(),
            &rules,
            TokenType::TypeSeparator
        ));
        assert!(check_single_token_parse(
            RANGE_SEPERATOR_LITERAL.to_string(),
            &rules,
            TokenType::RangeSeparator
        ));
        assert!(check_single_token_parse(
            OPEN_PARAM_LITERAL.to_string(),
            &rules,
            TokenType::OpenParam
        ));
        assert!(check_single_token_parse(
            CLOSE_PARAM_LITERAL.to_string(),
            &rules,
            TokenType::CloseParam
        ));
        assert!(check_single_token_parse(
            OPEN_BRACKET_LITERAL.to_string(),
            &rules,
            TokenType::OpenBracket
        ));
        assert!(check_single_token_parse(
            CLOSE_BRACKET_LITERAL.to_string(),
            &rules,
            TokenType::CloseBracket
        ));
    }

    #[test]
    fn test_number_literal_ufloat_token() {
        let rules = alloc_rules();
        for i in 0..1000 {
            let num = random::<f64>().to_string();
            assert!(check_single_token_parse(
                num.clone(),
                &rules,
                TokenType::NumberLiteral(num, Float)
            ))
        }
    }
    #[test]
    fn test_number_literal_all_float_token() {
        let rules = alloc_rules();

        for i in 0..1000 {
            let mut rng = rand::rng();
            let num = rng.random_range(-f32::MIN..=f32::MAX).to_string();
            assert!(check_single_token_parse(
                num.clone(),
                &rules,
                TokenType::NumberLiteral(num, Float)
            ))
        }
    }
}
