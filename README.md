# nexus-lang

The Nexus programming language.

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
func free(a, b, c) {
  return 42;
}

func main() {
  func local() {
    return "Hello".length();
  }

  free(1, 2, local());

  return func { return 17; }()
}
```

## Example 2

```
// Comment.
func free(a, b, c) {
  ret 42;
}

func main() {
  func local() {
    ret "Hello".length();
  }

  free(1, 2, local());

  ret func { ret 17; }()
}
```


# Ideas

* Make Nexus more expression oriented (e.g. make `for`/`while`/etc. statements expressions).
