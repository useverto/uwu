#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind};
    use uwu::parser::Parser;
    use uwu::scanner::{Diagnostic, Scanner, ScannerErrorKind};

    fn check(source: &str) -> Vec<Diagnostic> {
        let mut scanner = Scanner::new();
        let ast = Parser::new("<test>", source).parse();
        scanner.scan(ast);
        scanner.diagnostics()
    }

    macro_rules! diagnostic {
        ($kind: expr, $loc: expr, $msg: expr) => {
            Diagnostic::new($kind, $loc, $msg.to_string())
        };
    }

    #[test]
    fn computed_member_expr() {
        assert_eq!(
            check(r#"window["ev"+ "al"]()"#),
            vec![
                diagnostic!(
                    ScannerErrorKind::ComputedMemberExpr,
                    (0, 18),
                    "Computed member expression are not allowed."
                ),
                diagnostic!(
                    ScannerErrorKind::ItemNotFound,
                    (0, 6),
                    "Item not found in scope."
                )
            ]
        );
    }
}
