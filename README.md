# nexus-lang

The Nexus programming language.

Nexus is a language for aiding in software component network descriptions.
Aside a simple base of common general-purpose primitives/control flow/etc. it offers native integration for building a network of components, connecting in-/outputs and setting component properties.
The syntax of Nexus is loosely modeled after that of the [Rust programming language](https://github.com/rust-lang/rust).

**NOTE: This project is still very much under construction -- don't use in production!**

## Native integration with component networks

Nexus is designed to interface component-network-oriented systems:

```rust
// Instantiate components:
let c1 = node("TypeA");
let c2 = node("TypeB");

let mut system = mod;
system.source = c1;
system.sink   = c2;

let system.processor = node("TypeC"); // Ad-hoc definitions.

// References to (sub-)systems:
let proc = &system.processor;
proc.velocity = 3.14;

// Operators for defining edge connections:
c1.Output -> system.processor.Input;
system.processor.Output -> c2.Input;
```

## Simplicity

Nexus is geared towards simplicity, in the sense that it tries to support a minimal viable set of features required for flexible use as a component network description language.
It does not

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
let mut x;

x = true;

let y = x; // OK.
```

#### Argumentation

Using values uninitialized (and having the interpreter assume a value) is an error in 99.99% of the cases.

Also, this initialization requirement implicitly assumes a variable to be declared (which is not required even in languages like JavaScript).
Using a value undeclared is an error in 99.99% of the cases.

### Block scopes are strictly required

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

## Tooling

One of the focus points of Nexus is that there should be good tooling.
This has many aspects:

- Tools may come from other languages: due to Rust-like syntax, `rustfmt` should work.
- Nexus should be friendly for building tools for; the API should be simple.
- Nexus should (on the long run) be delivered with tools.

## Documentation

...

## The component model

...

## Language front-end API

..how should visual tools interact with Nexus?

## Language backend-end API / FFI

..how should software component networks interact with Nexus?

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

### Fundamental data types

Nexus is dynamically typed.
All variables are declared using `let` (immutable, directly initialized) or `let mut` (mutable) and are typed according to first initialization.
After first use, the type is strictly checked.

There are three fundamental data types:

- `String`, a Unicode string,
- `Number`, a double-precision (64 bits), signed floating-point number,
- `bool`, a boolean logic value.

### Expressions

- Loop: `while`/`for`/`loop`
- Conditional: `if`/`match`
- Closure: `|x|{ /* ... */ }`
- Range: `x..y` or `x..=y`

### Statements

- Declaration: `let`
- Function: `fn`
- Return: `return`
- Print: `print`
- Expression statements

## Implementation status

### Milestone 0: ideation, base setup

- [x] Lexing/scanning setup.
- [x] Scanner error handling.
- [x] Interpretation from source file (`.nxs`).
- [x] Command-line REPL setup.
- [ ] Review language design and setup (README).

### Milestone 1: language setup

- [ ] First thorough iteration of grammar rules.
- [ ] Parsing setup.
- [ ] Parsing error handling.
- [ ] Debugging commands in REPL and interpreter.

### Milestone 2: foundations

- [ ] Fundamental type `Number`.
- [ ] Fundamental type `String`.
- [ ] Simple arithmetic expressions.
- [ ] Printing of values.

### Milestone 3: basics

- [ ] Functions using `fn`.
- [ ] ...

## Keywords

### Base language keywords

| Keyword | Description |
| :-----: | :---------- |
| `fn`     | Function declaration.   |
| `for`    | Loop expression.        |
| `if`     | Conditional expression. |
| `let`    | Variable declaration.   |
| `loop`   | Loop expression.        |
| `match`  | Match expression.       |
| `return` | Return statement.       |
| `use`    | External use statement. |
| `while`  | Loop expression.        |

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
- `use` to include same-named `.nxs` files within visibility.
- Component instantiation using `node`
- Node edge connection using binary `->`
- Assignment
- Proper precedence handling

## TODO

- Improve declarative approach for extending a module with components.
- Immutability? Just shallow mutability for now. Add `mut` keyword.
- Objects?
- Object literal notation? (or JSON literal notation)
- Add `match` expression? Should be simple for a few fundamental types.
- Handling setting of component values...how/what/mutability?
- Lambda expressions?
- Move semantics as a default?
- Implicit return value (to remove `return`)?
- Error handling? Result types?
- Support for integration into a visual IDE / generative tooling.
- Range declarations using `start..=end` for use in `for` loops.
- Require safe edge types? How?
- Provide clear and good error messages on every level.
- FFI? How to deal with FFI of rich Unicode strings?

## FAQ

### Why the name?

From the dictionary:

**Nexus**; *nex·us*; meaning: *connection, link*

Of course this ties back to its place as a component network-description language.
