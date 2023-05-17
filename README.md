# nexus-lang

[![MIT licensed](https://img.shields.io/github/license/krisvanrens/nexus-lang?style=for-the-badge)](./LICENSE)

The Nexus programming language.

Nexus is a language for supporting software component network descriptions.
Aside a simple base of common general-purpose primitives/control flow/etc. it offers native integration for building a network of components, connecting in-/outputs and setting component properties.
The syntax and most semantics of Nexus are loosely modeled after that of the excellent [Rust programming language](https://github.com/rust-lang/rust).

Nexus is meant to drive a software component network system through its API, using the Nexus network description as input.

**NOTE**: This project is still very much under construction -- anything might change!

## Native integration with component networks

Nexus is designed to interface component-network-oriented systems:

```rust
// Instantiate components:
let c1 = node "TypeA";
let c2 = node "TypeB";

let mut system : group; // A component group named 'system'.
system.source = c1;
system.sink = c2;

let system.processor = node "TypeC"; // Ad-hoc definitions.

// References to (sub-)systems:
let proc = &system.processor;
proc.velocity = 3.14;

// Operators for defining edge connections:
c1.Output -> system.processor.Input;
system.processor.Output -> c2.Input;
```

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
let mut x;

x = true;

let y = x; // OK.
```

#### Argumentation

Using values uninitialized (and having the interpreter assume a value) is an error in 99.99% of the cases.

Also, this initialization requirement implicitly assumes a variable to be declared (which is not even required in languages like JavaScript).
Using a value undeclared always is an error.

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
Consider this C code as an example:

```c
if (expr)
    printf("A\n");
    printf("B\n"); // Whoops.
```

This will always print `B`, regardless of the `condition`.
However, due to the simple, unrelated matter of code formatting, it could easily be overlooked by a reviewer as a bug.

## Tooling

One of the focus points of Nexus is that there should be good tooling.
This has many aspects:

- Nexus should be friendly for building tools for; the API should be simple.
- Nexus should (on the long run) be delivered with tools and examples.

## Documentation and tests

As soon as the language syntax and semantics settle, documentation will be added.
The current leading implementatation of Nexus, `nexus_rs` will be documented and tested thoroughly.

## The component model

This section describes the abstract component model used to define networks for.

`// TODO`

## Language API

This section describes the API used to interact with a software component network decription system.
It can also be used by other tools, e.g. a visualizer for networks described by Nexus.

`// TODO`

## Language backend-end API / FFI

..how should software component networks interact with Nexus?

## Examples

### Example 1: general purpose

```rust
// Comment.
fn free(a: Number, b: bool, c: Number) -> Number {
    42 + a + if b { c } else { 0 }
}

/* Comment */
fn main() {
    fn local1() {
        return "Hello1".len();
    }

    let local2 = ||{ return "Hello2".len(); };

    let x = free(1, true, local1() + local2());

    print |x|{ 17 + x }(); // IIFE.
}
```

### Example 2: graph description

```rust
fn create_system(name: String) -> group {
    let mut sys = group(name);

    let sys.source = node "Reader";
    let sys.sink = node "Writer";

    sys.source.Output -> sys.sink.Input;

    // Group in-/outputs:
    let sys.Input = &sys.source.Input;
    let sys.Output = &sys.sink.Output;

    sys
}

let mut app : group;

// Create four systems in 'app':
for i in 0..4 {
    app[i] = create_system("Sys" + i);
}

print app;
```

## Feature list

### Fundamental data types

Nexus is dynamically typed.
All automatic variables are declared using `let` (immutable, directly initialized) or `let mut` (mutable) and are typed according to first initialization.
After first use, the type is strictly checked.
Function arguments are always strictly typed.

There are three fundamental data types:

- `String`, a Unicode string,
- `Number`, a double-precision (> 64 bits), signed floating-point number,
- `bool`, a boolean logic value.

### Expressions

- Loop: `while`/`for`
- Conditional: `if`
- Closure: `|x|{ /* ... */ }`
- Range: `x..y` (exclusive) or `x..=y` (inclusive)

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

- [ ] Including other files with `use`
- [ ] Functions using `fn`.
- [ ] ...

T.B.D.

## Keywords

### Language type keywords

| Keyword | Description |
| :-----: | :---------- |
| `bool`   | Boolean logic type. |
| `Number` | Number type.        |
| `String` | String type.        |

### Base language keywords

| Keyword | Description |
| :-----: | :---------- |
| `const`  | Constant declaration.          |
| `fn`     | Function declaration.          |
| `for`    | Loop expression.               |
| `if`     | Conditional expression.        |
| `let`    | Variable declaration.          |
| `mut`    | Variable mutability specifier. |
| `return` | Return statement.              |
| `use`    | External use statement.        |
| `while`  | Loop expression.               |

### Language library keywords

| Keyword | Description |
| :-----: | :---------- |
| `group` | Component group instantiation. |
| `node`  | Component instantiation.       |
| `print` | Print expression result.       |

## Language grammar

Productions are in [Extended Backus-Naur Form (EBNF)](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).

### Lexical grammar

```ebnf
ALPHA  = "a" | "..." | "z" | "A" | "..." | "Z" | "_" ;
DIGIT  = "0" | "..." | "9" ;
STRING = "\"" , <character>* - "\"" , "\"" ;
NUMBER = DIGIT+ ( "." DIGIT+ )? ;
ID     = ALPHA ( ALPHA | DIGIT )* ;
```

Note: for simplicity in the production rules, `ALPHA` is represented here as ASCII alphabetic.
However, in `nexus-rs`, it means any *alphabetic* character, as defined by [chapter 4](https://www.unicode.org/versions/Unicode15.0.0/ch04.pdf) of [Unicode standard](https://www.unicode.org/versions/Unicode15.0.0/).
This means in practice it is possible to define identifiers named `ŮñĭçøƋɇ`.

### Main syntax (WIP)

Note: the grammar will be extended as the language implementation progresses.

```ebnf
program    = decl* EOF ;

type       = "bool" | "Number" | "String" ;
decl       = fn_decl | const_decl | var_decl | use_decl | stmt ;
fn_decl    = "fn" function ;
const_decl = "const" ID ":" type "=" expr ";" ;
var_decl   = "let" ( "mut" )? ID ( ( "=" expr ) | ( ":" type ) ( ":" type "=" expr ) )? ";" ;
use_decl   = "use" expr ";" ;
stmt       = expr_stmt | print | return | block ;
expr_stmt  = expr ";" ;
print      = "print" expr ";" ;
return     = "return" expr ";" ;
block      = "{" decl* "}" ;

function   = ID "(" params* ")" ( "->" type )? block ;
params     = ID ":" type ( "," ID ":" type )* ;
args       = expr ( "," expr )* ;

literal    = NUMBER | STRING | "true" | "false" ;
expr       = literal | unary | binary | group ;
unary      = ( "!" | "+" | "-" | "group" | "node" ) expr ;
operator   = "==" | "!=" | "<=" | ">=" | "<" | ">" | "||" | "&&" | "+" | "-" | "*" | "/" | "%" ;
binary     = expr operator expr ;
group      = "(" expr ")" ;
```

#### Glossary

| Abbreviation | Meaning |
| :----------: | :------ |
| `decl` | Declaration |
| `expr` | Expression  |
| `stmt` | Statement   |

#### `// TODO`

- Assignment (stmt)
- Function call (expr)
- `for` (expr)
- `if`/`else` (expr)
- `while` (expr)
- Closures (expr)
- Ranges (expr)

## Known limitations

- Due to the current line-based scanning implementation, only a single scanning error per line will be detected. This is fine for now.

## TODO

- Improve declarative approach for extending a module with components.
- Simple suport for variable aliases (references)? Should be handy for shorthand names.
- Immutability? Is the benefit of immutibility by default + move semantics beneficial for the use case of Nexus? Why or why not?
- Support for objects? Groups using `group` should suffice.
- Execution entry point? Just structural starting from the root `.nxs` file?
- Object literal notation? (or JSON literal notation)
- Add `match` expression? Should be simple for a few fundamental types.
- Handling setting of component values...how/what/mutability?
- Implicit return value (to omit `return` in most places)?
- Add combined assigment/operators (`+=`/`-=`/`*=`/`/=`).
- Traits for fundamental types? E.g. `"sdfs".len() == 4` etc.
- Is it possible to have `Number` be floating-point when sometimes used as integer?
- Error handling? Result types?
- Support for integration into a visual IDE / generative tooling.
- Require safe edge types? How? Should be dealt with in the API -- possibly a responsibility of the component network integration.
- Provide clear and good error messages on every level.
- FFI? How to deal with FFI of rich Unicode strings?
- Add `loop` expression? This will also require `break` and `continue` (which would be nice anyway..).
- What is the difference between the front- and backend API? Is there a difference at all? What are the needs for a visualization tool vs. those of the component network integration itself?
- Tool idea: Nexus to Graphviz Dot description.
- Example integrations and implementations. Nexus to Rust library, Nexus to C++ library etc.
- Shadowing like Rust does?
- Enforce style: upper snake case for `const`?
- Function order is arbitrary like Rust?
- `if` expressions _must_ be of type `bool`.
- `else if` support.
- Printing of `group`s and `node`s.
- Generating an AST graph image for debugging.
- Underscore for integer number separators?

## FAQ

### Why the name?

From the dictionary:

> **Nexus**; *nex·us*; meaning: *connection, link*

Of course this ties back to its place as a component network-description language.

