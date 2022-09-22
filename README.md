# nexus-lang

The Nexus programming language.

## Native integration with component networks

Nexus is meant to interface component-network-oriented systems, to enable a convenient, safe network description.

```
// Instantiate components:
var c1 = comp("MyComponentTypeA");
var c2 = comp("MyComponentTypeB");

// Simple operators for defining connections, and native output names on component types.
c1.Output -> c2.Input;
```

## Safety

Nexus is opinionated in some respects, mostly to improve safety.
The following subsections indicate in what specific ways.

### Using a value uninitialized is an error

Even though Nexus is dynamically typed, it requires a value to be initialized when it's used.
E.g.:

```
var x;

if (x) { /* ... */ } // ERROR: Using 'x' uninitialized.
```

and:

```
var x;

x = true;

if (x) { /* ... */ } // OK.
```

#### Argumentation

Using values uninitialized (and having the interpreter assume a value) is an error in 99.9% of the cases.

Also, this initialization requirement implicitly assumes a variable to be declared (which is not required even in languages like JavaScript).
Using a value undeclared is an error in 99.9999999% of the cases.

### Braces are strictly required

Braces after `if`/`while`/`for`/etc. statements are required:

```
if (...) {
  // OK.
}

// Other code..

if (...) // ERROR: Expecting braces for block statement.

// Other code..
```

In other words, only block statements are allowed.

#### Argumentation

The argumentation for strictly requiring braces is simple: this prevents statement blocks to be "cut up" accidentally.
Consider this example C code as an example:

```c
if (condition)
  printf("a\n");
  printf("b\n");
```

This will always print `b`, regardless of the `condition`.
However, due to the simple, unrelated matter of code formatting, it could be overlooked by a reviewer as a bug.


# Examples

## Example 1

```
// Comment.
fun free(a, b, c) {
  return 42;
}

fun main() {
  fun local {
    return "Hello".length();
  }

  free(1, 2, local());

  ret fun { ret 17; }();
}
```


# Ideas

* Make Nexus more expression oriented (e.g. make `for`/`while`/etc. statements expressions).
