use rlox::{interpreter::Interpreter, parser::Parser, scanner::Scanner, value::LoxValue};

#[allow(clippy::nonminimal_bool)]
fn test_cases() -> Vec<(&'static str, LoxValue)> {
    vec![
        // literals
        ("nil", LoxValue::Nil),
        ("true", LoxValue::Bool(true)),
        ("false", LoxValue::Bool(false)),
        ("123.45", LoxValue::Number(123.45)),
        ("-123.45", LoxValue::Number(-123.45)),
        ("\"hello lox\"", LoxValue::String("hello lox".to_owned())),
        // unary
        ("-123.45", LoxValue::Number(-123.45)),
        ("!true", LoxValue::Bool(false)),
        // binary
        ("123.45 + 54.321", LoxValue::Number(123.45 + 54.321)),
        ("123.45 - 54.321", LoxValue::Number(123.45 - 54.321)),
        ("123.45 * 54.321", LoxValue::Number(123.45 * 54.321)),
        ("123.45 / 54.321", LoxValue::Number(123.45 / 54.321)),
        (
            "\"hello\" + \" lox\"",
            LoxValue::String("hello lox".to_owned()),
        ),
        ("123.45 > 54.321", LoxValue::Bool(123.45 > 54.321)),
        ("123.45 >= 54.321", LoxValue::Bool(123.45 >= 54.321)),
        ("123.45 < 54.321", LoxValue::Bool(123.45 < 54.321)),
        ("123.45 <= 54.321", LoxValue::Bool(123.45 <= 54.321)),
        ("123.45 == 54.321", LoxValue::Bool(123.45 == 54.321)),
        ("123.45 != 54.321", LoxValue::Bool(123.45 != 54.321)),
        ("true == nil", LoxValue::Bool(false)),
        ("false != nil", LoxValue::Bool(true)),
        ("true == 1.0", LoxValue::Bool(false)),
        ("true != 1.0", LoxValue::Bool(true)),
        // ternary
        ("1 < 2 ? 1 : 2", LoxValue::Number(1.0)),
        ("1 > 2 ? 1 : 2", LoxValue::Number(2.0)),
        // Lox is a dynamic type language, so it's ternary expression can return union type value.
        // In the following situation, the return type of ternary operator is `number | string`.
        ("1 < 2 ? 1 : \"abc\"", LoxValue::Number(1.0)),
        ("1 > 2 ? 1 : \"abc\"", LoxValue::String("abc".to_owned())),
        // grouping
        ("!(1 > 2)", LoxValue::Bool(!(1 > 2))),
        ("-((1 + 2) * 3)", LoxValue::Number(-((1.0 + 2.0) * 3.0))),
        (
            "(1 + 2) * 3 / ((6 - 2) * 1 / 2)",
            LoxValue::Number((1.0 + 2.0) * 3.0 / ((6.0 - 2.0) * 1.0 / 2.0)),
        ),
    ]
}

#[test]
fn test_evaluation() {
    let mut interpreter = Interpreter::new();
    test_cases().into_iter().for_each(|(input, expected)| {
        let mut scanner = Scanner::new(input.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();
        assert!(program.is_some());
        let evaluated = interpreter.inperpret(program.unwrap());
        assert!(evaluated.is_some());
        assert_eq!(expected, evaluated.unwrap());
    });
}
