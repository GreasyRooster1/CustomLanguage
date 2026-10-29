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

        let mut interpreter = Interpreter::new();
        interpreter.execute(*program);

        println!("{:#?}",interpreter)
    }

    #[test]
    fn test_interpreter_debugs() {
        let parsed = parse(include_str!("../../lang/InterpreterFunctionsTest.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let mut interpreter = Interpreter::new();
        println!("{:#?}",interpreter);
        let output_val = interpreter.execute(*program).unwrap();

        println!("{:#?}",output_val)
    }

    #[test]
    fn test_interpreter_loops() {
        let parsed = parse(include_str!("../../lang/InterpreterLoopsTest.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let mut interpreter = Interpreter::new();
        println!("{:#?}",interpreter);
        let output_val = interpreter.execute(*program).unwrap();

        println!("{:#?}",output_val)
    }

    #[test]
    fn test_interpreter_for_loops() {
        let parsed = parse(include_str!("../../lang/InterpreterForLoopsTest.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let mut interpreter = Interpreter::new();
        println!("{:#?}",interpreter);
        let output_val = interpreter.execute(*program).unwrap();

        println!("{:#?}",output_val)
    }

    #[test]
    fn test_interpreter_math() {
        let parsed = parse(include_str!("../../lang/InterpreterMathTest.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let mut interpreter = Interpreter::new();
        println!("{:#?}",interpreter);
        let output_val = interpreter.execute(*program).unwrap();

        println!("{:#?}",output_val)
    }

    #[test]
    fn test_interpreter_ret() {
        let parsed = parse(include_str!("../../lang/InterpreterRetTest.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let mut interpreter = Interpreter::new();
        println!("{:#?}",interpreter);
        let output_val = interpreter.execute(*program).unwrap();

        println!("{:#?}",output_val)
    }

    #[test]
    fn test_interpreter_std() {
        let parsed = parse(include_str!("../../lang/InterpreterStdTest.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let mut interpreter = Interpreter::new();
        println!("{:#?}",interpreter);
        let output_val = interpreter.execute(*program).unwrap();

        println!("{:#?}",output_val)
    }

    #[test]
    fn test_interpreter_types() {
        let parsed = parse(include_str!("../../lang/InterpreterTypesTest.dte").to_string());
        println!("{:?}",parsed);
        let mut ast = AST::new(parsed);
        let node = ast.parse_program();
        let Node::ProgramNode(program) = node else { panic!("Expected program node, got {:?}", node) };
        println!("{:#?}",program);

        let mut interpreter = Interpreter::new();
        println!("{:#?}",interpreter);
        let output_val = interpreter.execute(*program).unwrap();

        println!("{:#?}",output_val)
    }

}
