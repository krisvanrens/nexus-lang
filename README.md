# nexus-lang

The Nexus programming language.

## Native integration with component networks

Nexus is designed to interface component-network-oriented systems, to enable a convenient, safe network description.

```rs
// Instantiate components:
let c1 = node("TypeA");
let c2 = node("TypeB");

// Operators for defining edge connections:
c1.Output -> c2.Input;
```

From a syntactic viewpoint, it is mostly influenced by [Rust](https://github.com/rust-lang/rust).

## Simplicity

Nexus is geared towards simplicity, in the sense that it tries to support a minimal viable set of features required for flexible use as a component network description language.

## Safety

Nexus is opinionated in some respects, mostly to improve safety.
The following subsections indicate in what specific ways.

### Using a value uninitialized is an error

Even though Nexus is dynamically typed, it requires a value to be initialized when it's used.
E.g.:

```rust
let x;

let y = x; // Error: Using 'x' uninitialized.
```

and:

```rust
let x;

x = true;

let y = x; // OK.
```

#### Argumentation

Using values uninitialized (and having the interpreter assume a value) is an error in 99.99% of the cases.

Also, this initialization requirement implicitly assumes a variable to be declared (which is not required even in languages like JavaScript).
Using a value undeclared is an error in 99.99% of the cases.

### Braces are strictly required

Braces after `if`/`while`/`for`/etc. statements are required:

```js
// OK:
if expr {
  do_something();
}

// Error:
if expr
  do_something();
```

In other words, only block statements are allowed.

#### Argumentation

The argumentation for strictly requiring braces is simple: this prevents statement blocks to be "cut up" accidentally.
Consider this example C code as an example:

```c
if (condition)
  printf("A\n");
  printf("B\n"); // Whoops.
```

This will always print `B`, regardless of the `condition`.
However, due to the simple, unrelated matter of code formatting, it could be overlooked by a reviewer as a bug.

## Examples

### Example 1

```rust
// Comment.
fn free(a, b, c) -> Int {
  return 42 + a + b + c;
}

/* Comment */
fn main() {
  fn local1() {
    return "Hello1".length();
  }

  let local2 = ||{ return "Hello2".length(); };

  let x = free(1, 2, local1() + local2());

  print |x|{ return 17 + x; }();
}
```

## Feature list

### Expressions

- Loops: `for`/`loop`
- Conditions: `if`/`match`

### Statements

- Declaration: `var`/`let`

## Implementation status

### Milestone 0: setup

- [ ] Lexing setup.
- [ ] Parsing setup.

### Milestone 1: foundations

- [ ] Fundamental type `Number`.
- [ ] Fundamental type `String`.
- [ ] Simple arithmetic expressions.
- [ ] Printing of values.
- [ ] Interpretation from source file (`.nxs`).
- [ ] Command-line REPL.

### Milestone 2: basics

- [ ] Functions using `fn`.
- [ ] ...

## Keywords

### Base language keywords

| Keyword | Description |
| :-----: | :---------- |
| `fn`    | Function declaration.   |
| `for`   | Loop expression.        |
| `if`    | Conditional expression. |
| `let`   | Variable declaration.   |
| `loop`  | Loop expression.        |
| `match` | Match expression.       |
| `while` | Loop expression.        |

### Language library keywords

| Keyword | Description |
| :-----: | :---------- |
| `print` | Print expression result. |
| `node`  | Component instantiation. |

## Language grammar

Productions in [Extended Backus-Naur Form (EBNF)](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).

### Lexical grammar

```ebnf
ALPHA  = "a" | "..." | "z" | "A" | "..." | "Z" | "_" ;
DIGIT  = "0" | "..." | "9" ;
STRING = "\"" , <character>* - "\"" , "\"" ;
NUMBER = DIGIT+ ( "." DIGIT+ )? ;
ID     = ALPHA ( ALPHA | DIGIT )* ;
```

### Main syntax (WIP)

```ebnf
program = decl* EOF ;

decl       = fn_decl | var_decl | stmt ;
fn_decl    = "fn" function ;
var_decl   = "let" ID ( "=" expr )? ";" ;
stmt       = expr_stmt | print_stmt | block ;
expr_stmt  = expr ";" ;
print_stmt = "print" expr ";" ;
block      = "{" decl* "}" ;

function   = ID "(" params* ")" block ;
params     = ID ( "," ID )* ;
args       = expr ( "," expr )* ;

literal    = NUMBER | STRING | "true" | "false" ;
expr       = literal | unary | binary | group ;
unary      = ( "!" | "-" ) expr ;
operator   = "==" | "!=" | "<=" | ">=" | "<" | ">" | "+" | "-" | "*" | "/" ;
binary     = expr operator expr ;
group      = "(" expr ")" ;
```

### TODO

Add:

- `for .. in`
- `while`
- `loop`
- `match`
- Component instantiation using `node`
- Node edge connection using binary `->`
- Assignment
- Proper precendence handling

## TODO

- Immutability? Just shallow mutability for now.
- Fundamental types:
  - `Number`: number (double-precision floating-point, 64-bits)
  - `String`: string (UTF-8 string)
- Objects?
- Add `match` expression? Should be simple for a few fundamental types.
- Handling setting of component values...how/what/mutability?
- Lambda expressions?
- Copy/move semantics?
- Implicit return value (to remove `return`)?
- Support for JSON literal object notation?
- Error handling? Result types?
- Support for integration into a visual IDE / generative tooling.
- Range declarations using `start..=end` for use in `for` loops.

## Known limitations

- The scanner does not support in-line comments in all places, e.g. in between an empty closure declaration and the body, as in: `|| /* ... */ {}`.

## FAQ

### Why the name?

From the dictionary:

**Nexus**; *nex·us*; *Connection, link*

Of course this ties back to its place as a component network-description language.

