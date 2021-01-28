#[macro_export]
macro_rules! create_diagnostic {
    ($src: expr, $code: expr, $span: expr, $err: expr, $hint: expr) => {
        format!(
            r#"
error[ERRCODE]: {err}
--> {src}:{line}:{col}
    |
{line}   | {code}
    | {underline} {hint}
    "#,
            src = $src,
            line = $span[0],
            col = $span[1],
            err = $err,
            hint = $hint,
            code = $code,
            underline = $crate::diagnostics::underline($span[1]),
        )
    };
}

pub fn underline(col: usize) -> String {
    let mut s = String::new();
    for _ in 0..col {
        s.push_str("-");
    }
    s
}

#[cfg(test)]
mod diagnostics_test {
    use crate::create_diagnostic;

    #[test]
    fn test_minimal_create() {
        assert_eq!(
            create_diagnostic!(
                "test.uwu",
                "window",
                [2, 2],
                "no item named `window` found in scope.",
                "associated item `window` is not declared"
            ),
            r#"
error[ERRCODE]: no item named `window` found in scope.
--> test.uwu:2:2
    |
2   | window
    | -- associated item `window` is not declared
    "#
        );
    }
}
