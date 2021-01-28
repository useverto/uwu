# uwu

uwu. Transpiles to safe, optimised &amp; pluggable JavaScript.

Let's say you want your user's to enter code. This can be due to various reasons including plugins, customizations, etc.

The problem is, _should you accept/run Javascript from an untrusted source?_ Probably not.
```js
// This is dangerous!
let key = localStorage.getItem("apiKey");
```

With uwu language design, code is completely sandboxed from it's outer enviornment.
The following code won't compile at all:
```js
let key = localStorage["getItem"]("apiKey");
```

```
error: no item named `localStorage` found in scope.
--> test.uwu:1:11
    |
11  | localStorage["getItem"]("apiKey")
    | -- associated item `localStorage` is not declared
```

## Syntax

### `variables`

Syntax for variable declaration and mutation is equivalent to that of ECMAScript.

```js
let i = 0;
i = 1;
```

### `types`

```py
# string
"This is a String"

# boolean
true 
false

# numbers
1
3.14

# array
[1, "hello", ["nested", 2]]

# object/dicts
{"key": "value"}
```

### `if...else`

```lua
if(condition):
  ...
else:
  ...
end
```

### `while`

```lua
while(condition):
  ...
end
```


### `functions`

```rust
fn add(x, y):
  return x + y
end

add(1, 2)
```

## Integration

The compiler internals are written in Rust and compiled to WASM for simple client-side browser integration.

```typescript
import { compile } from "@useverto/uwu";

let [result, diagnostics] = compile("let num = 1");

// Check for compiler diagnostics
if(diagnostics.length > 0) {
    // throw the first diagnostics to user
    throw new Error(diagnostics[0])
} else {
    // evaluate the result
    // or do whatever with it
    let execute = new Function(result);
    execute();
}
```

You can also directly use the compiler from Rust.

```toml
[dependencies]
uwu = { git = "https://github.com/useverto/uwu" }
```

```rust
use uwu::{
    tokenizer::Lexer,
    parser::Parser,
    compiler::Compiler,
};

fn compile(source: &str) -> Result<String, String> {
    let mut parser = Parser::new(Lexer::new(source));
    let ast = parser.parse();
    let errs = parser.get_errors();
    if errs.len() > 0 {
        let e = &errs[0];
        return Err(create_diagnostic!(
            "<anon>",
            source.chars().nth(e.current_token.loc - 1).unwrap(),
            [e.current_token.loc, e.current_token.loc],
            e.msg,
            e.msg
        ));
    }
    let compiler = Compiler::new(ast);
    Ok(compiler.compile())
}

// Use the compile function anywhere
compile("1 + 1")?;
```

## License

