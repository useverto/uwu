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

    macro_rules! fixture_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let expected_diagnostics: Vec<Diagnostic> = serde_json::from_str(expected).unwrap();
                assert_eq!(check(input), expected_diagnostics);
            }
        )*
        }
    }

    fixture_tests! {
        none_0: (include_str!("../cases/none.js"), include_str!("../cases/none.d.json")),

        iife_0: (include_str!("../cases/iife.000.js"), include_str!("../cases/iife.000.d.json")),
    }
}
