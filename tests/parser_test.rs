use rlox::{ast::pretty_printer::AstPrinter, parser::Parser, scanner::Scanner};

fn preprae_test_case() -> Vec<(&'static str, &'static str)> {
    vec![
        ("1 + 2 * 3", "(+ 1 (* 2 3))"),
        ("1 + 2 * 3 - 4 / 5", "(- (+ 1 (* 2 3)) (/ 4 5))"),
        ("-1 + 2", "(+ (- 1) 2)"),
        ("1 != 2 + 3", "(!= 1 (+ 2 3))"),
        (
            "(-1 + 2) * 3 == 6 / 2 == true != false != nil",
            "(!= (!= (== (== (* (group (+ (- 1) 2)) 3) (/ 6 2)) true) false) nil)",
        ),
        ("\"hello\" + \"lox\"", "(+ hello lox)"),
        (
            "1 + 2 == 3 / 4 ? 2 - 1 : 5 + 6 * 7",
            "(? (== (+ 1 2) (/ 3 4)) (- 2 1) (+ 5 (* 6 7)))",
        ),
        (
            "1 == 2 ? 3 : 4 == 5 ? 6 : 7",
            "(? (== 1 2) 3 (? (== 4 5) 6 7))",
        ),
    ]
}

#[test]
fn test_parser() {
    let test_cases = preprae_test_case();
    let mut printer = AstPrinter();
    test_cases.iter().for_each(|(source, expected)| {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expected, &printer.fmt(&expr));
    });
}
