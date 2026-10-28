#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::ast::*;
    use crate::interpreter::{Interpret, Interpreter};
    use crate::lexer::lexer::parse;

    #[test]
    fn test_interpreter_execution() {
        let parsed = parse(include_str!("../../lang/ASTProgramTest1.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let interpreter = Interpreter::new();
        interpreter.execute(*program);
    }

}
