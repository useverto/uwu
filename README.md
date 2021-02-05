# uwu

uwu. Transpiles to safe, optimised &amp; pluggable JavaScript.

> This is pre-alpha software. Expect breaking changes.

Let's say you want your users to enter code. This can be due to various reasons
including plugins, customizations, etc.

The problem: _Should you accept/run Javascript from an untrusted source?_
Probably not.

```js
// This is dangerous!
let key = localStorage.getItem("apiKey");
```

With uwu's language design, code is completely sandboxed from its outer
enviornment. The following code won't compile at all:

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

### Syntax

#### `variables`

Syntax for variable declaration and mutation is equivalent to that of
ECMAScript.

```js
let i = 0;
i += 1;
```

#### `types`

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

# regexp
/[abc]/g
```

#### `if...else`

```lua
if(<expr>):
  [<expr>, ...]
else:
  [<expr>, ...]
end
```

### `while`

```lua
while(<expr>):
  [<expr>, ...]
end
```

#### `functions`

```rust
fn add(x, y):
  return x + y
end

add(1, 2)
```

## License

MIT License